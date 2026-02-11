use crate::metadata::LazarusHeader;
use crate::core::calculate_checksum;
use anyhow::{Result, bail, Context};
use std::io::Read;
use xz2::read::XzDecoder;
use wirehair_wrapper::wirehair::{WirehairDecoder, WirehairResult};
use rayon::prelude::*;

pub struct Decoder {
    password: Option<String>,
}

impl Decoder {
    pub fn new(password: Option<String>) -> Self {
        Self { password }
    }

    pub fn decompress(&self, compressed_data: &[u8], recovery_data: &[u8], header: &LazarusHeader) -> Result<Vec<u8>> {
        if &header.magic != b"LZRS" {
            bail!("Invalid file format: Magic bytes mismatch");
        }

        let is_healthy = self.check_compressed_integrity(compressed_data, header);
        
        let body_to_decompress = if is_healthy {
            // Use parallel decompression if chunk sizes are available
            let decompress_result = if !header.compressed_chunk_sizes.is_empty() {
                self.lzma_decompress_parallel(compressed_data, &header.compressed_chunk_sizes, header)
            } else {
                if header.is_encrypted {
                    bail!("Encrypted archive requires chunk boundaries for parallel decryption. File is incompatible or corrupted.");
                }
                // Fallback to single-stream decompression for backward compatibility
                self.lzma_decompress(compressed_data, header.original_size)
            };
            
            match decompress_result {
                Ok(data) => data,
                Err(_) => {
                    println!("  ⚠️ LZMA stream error despite healthy CRCs. Attempting Phoenix Repair...");
                    self.repair_body(compressed_data, recovery_data, header)?
                }
            }
        } else {
            println!("  ⚠️ Corruption detected in body. Initiating Phoenix Protocol...");
            self.repair_body(compressed_data, recovery_data, header)?
        };

        let chunk_size = header.block_size as usize;
        for (i, chunk) in body_to_decompress.chunks(chunk_size).enumerate() {
            let actual_crc = calculate_checksum(chunk);
            if let Some(expected_crc) = header.fingerprints.get(i) {
                if actual_crc != *expected_crc {
                    bail!("Block {} verification failed! DNA mismatch.", i);
                }
            }
        }

        Ok(body_to_decompress)
    }

    fn check_compressed_integrity(&self, data: &[u8], header: &LazarusHeader) -> bool {
        if !header.has_recovery || header.compressed_fingerprints.is_empty() {
            return true;
        }
        let wh_block_size = 1024;
        for (i, chunk) in data.chunks(wh_block_size).enumerate() {
            if let Some(expected) = header.compressed_fingerprints.get(i) {
                if calculate_checksum(chunk) != *expected {
                    println!("  CRC mismatch at compressed block {}", i);
                    return false;
                }
            }
        }
        true
    }

    fn lzma_decompress(&self, data: &[u8], original_size: u64) -> Result<Vec<u8>> {
        let mut decompressor = XzDecoder::new(data);
        let mut buffer = Vec::with_capacity(original_size as usize);
        decompressor.read_to_end(&mut buffer).map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(buffer)
    }

    fn lzma_decompress_parallel(&self, data: &[u8], chunk_sizes: &[usize], header: &LazarusHeader) -> Result<Vec<u8>> {
        // Split compressed data into chunks based on stored sizes
        let mut chunks = Vec::new();
        let mut offset = 0;
        for &size in chunk_sizes {
            if offset + size > data.len() {
                bail!("Invalid chunk size: exceeds data length");
            }
            chunks.push(&data[offset..offset + size]);
            offset += size;
        }

        // Prepare encryption key if needed
        let key = if header.is_encrypted {
            let pwd = self.password.as_ref()
                .context("This archive is encrypted. Please provide a password.")?;
            let salt = header.encryption_salt
                .context("Missing encryption salt in header.")?;
            Some(crate::core::derive_key(pwd, &salt))
        } else {
            None
        };

        // Decompress each chunk in parallel (and decrypt if needed)
        let decompressed_chunks: Result<Vec<Vec<u8>>> = chunks.par_iter()
            .enumerate()
            .map(|(i, chunk)| {
                let processed_chunk = if let Some(ref k) = key {
                    crate::core::decrypt_data(chunk, k, i as u32)?
                } else {
                    chunk.to_vec()
                };

                let mut decompressor = XzDecoder::new(&processed_chunk[..]);
                let mut buffer = Vec::new();
                decompressor.read_to_end(&mut buffer)
                    .map_err(|e| anyhow::anyhow!("LZMA decompression failed: {}", e))?;
                Ok(buffer)
            })
            .collect();

        let decompressed_chunks = decompressed_chunks?;
        let result: Vec<u8> = decompressed_chunks.into_iter().flatten().collect();
        
        if result.len() != header.original_size as usize {
            bail!("Decompressed size mismatch: expected {}, got {}", header.original_size, result.len());
        }
        
        Ok(result)
    }

    fn repair_body(&self, corrupted_body: &[u8], recovery_shield: &[u8], header: &LazarusHeader) -> Result<Vec<u8>> {
        if !header.has_recovery || recovery_shield.is_empty() {
             bail!("Repair failed: No recovery shield found.");
        }

        let wh_block_size = 1024;
        let decoder = WirehairDecoder::new(corrupted_body.len() as u64, wh_block_size);
        
        let chunks = corrupted_body.chunks(wh_block_size as usize);
        let total_source_blocks = chunks.len();

        println!("  Scanning {} compressed blocks...", total_source_blocks);
        let mut bad_blocks = 0;
        
        for (i, chunk) in chunks.enumerate() {
            let is_valid = if let Some(expected) = header.compressed_fingerprints.get(i) {
                calculate_checksum(chunk) == *expected
            } else {
                true 
            };

            if is_valid {
                // If last chunk is small, we must pad it to wh_block_size for the decoder
                if chunk.len() < wh_block_size as usize {
                    let mut padded = vec![0u8; wh_block_size as usize];
                    padded[..chunk.len()].copy_from_slice(chunk);
                    let _ = decoder.decode(i as u64, &padded, wh_block_size);
                } else {
                    let _ = decoder.decode(i as u64, chunk, wh_block_size);
                }
            } else {
                bad_blocks += 1;
            }
        }
        println!("  Found {} corrupted blocks. Reconstructing...", bad_blocks);

        for (i, chunk) in recovery_shield.chunks(wh_block_size as usize).enumerate() {
            let id = (total_source_blocks + i) as u64;
            let res = decoder.decode(id, chunk, wh_block_size)
                .map_err(|e| anyhow::anyhow!("Wirehair Error: {:?}", e))?;
            
            if res == WirehairResult::Success {
                println!("  ✨ Wirehair success!");
                break;
            }
        }

        let mut repaired_body = vec![0u8; corrupted_body.len()];
        decoder.recover(&mut repaired_body, corrupted_body.len() as u64)
             .map_err(|e| anyhow::anyhow!("Phoenix Failed: {:?}", e))?;
        
        println!("  ✅ Body repaired.");
        
        // Use parallel decompression if chunk sizes are available
        if !header.compressed_chunk_sizes.is_empty() {
            self.lzma_decompress_parallel(&repaired_body, &header.compressed_chunk_sizes, header)
        } else {
            if header.is_encrypted {
                bail!("Encrypted archive requires chunk boundaries for parallel decryption.");
            }
            self.lzma_decompress(&repaired_body, header.original_size)
        }
    }
}