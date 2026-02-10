use crate::metadata::LazarusHeader;
use crate::core::calculate_checksum;
use anyhow::{Result, bail};
use std::io::Read;
use xz2::read::XzDecoder;
use wirehair_wrapper::wirehair::{WirehairDecoder, WirehairResult};

pub struct Decoder;

impl Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decompress(&self, compressed_data: &[u8], recovery_data: &[u8], header: &LazarusHeader) -> Result<Vec<u8>> {
        if &header.magic != b"LZRS" {
            bail!("Invalid file format: Magic bytes mismatch");
        }

        let is_healthy = self.check_compressed_integrity(compressed_data, header);
        
        let body_to_decompress = if is_healthy {
            match self.lzma_decompress(compressed_data, header.original_size) {
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
        self.lzma_decompress(&repaired_body, header.original_size)
    }
}