use crate::metadata::LazarusHeader;
use crate::core::calculate_checksum;
use anyhow::{Result, bail, Context};
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

        // 1. Attempt Standard Decompression
        println!("  Attempting standard decompression...");
        // Validate compressed integrity first if possible
        let integrity_check = self.check_compressed_integrity(compressed_data, header);
        
        if integrity_check {
             match self.lzma_decompress(compressed_data, header.original_size) {
                Ok(data) => return self.verify_original(data, header),
                Err(_) => println!("  LZMA Error. Fallback to repair..."),
             }
        } else {
             println!("  ⚠️ Corruption detected in main body (CRC check). Initiating Phoenix Protocol...");
        }

        // 2. Repair
        match self.repair_and_decompress(compressed_data, recovery_data, header) {
            Ok(data) => self.verify_original(data, header),
            Err(e) => bail!("Decompression failed: {:?}", e),
        }
    }

    fn check_compressed_integrity(&self, data: &[u8], header: &LazarusHeader) -> bool {
        if !header.has_recovery || header.compressed_fingerprints.is_empty() {
            return true; // Assume good if no recovery info
        }
        let wh_block_size = 1024;
        for (i, chunk) in data.chunks(wh_block_size).enumerate() {
            if let Some(expected) = header.compressed_fingerprints.get(i) {
                if calculate_checksum(chunk) != *expected {
                    return false;
                }
            }
        }
        true
    }

    fn verify_original(&self, data: Vec<u8>, header: &LazarusHeader) -> Result<Vec<u8>> {
        let chunk_size = header.block_size as usize;
        for (i, chunk) in data.chunks(chunk_size).enumerate() {
            let actual_crc = calculate_checksum(chunk);
            if let Some(expected_crc) = header.fingerprints.get(i) {
                if actual_crc != *expected_crc {
                    bail!("Block {} verification failed! DNA mismatch.", i);
                }
            }
        }
        Ok(data)
    }

    fn lzma_decompress(&self, data: &[u8], original_size: u64) -> Result<Vec<u8>> {
        let mut decompressor = XzDecoder::new(data);
        let mut buffer = Vec::with_capacity(original_size as usize);
        decompressor.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn repair_and_decompress(&self, corrupted_body: &[u8], recovery_shield: &[u8], header: &LazarusHeader) -> Result<Vec<u8>> {
        if !header.has_recovery || recovery_shield.is_empty() {
             bail!("Repair failed: No recovery shield found.");
        }

        let wh_block_size = 1024;
        let decoder = WirehairDecoder::new(corrupted_body.len() as u64, wh_block_size);
        
        let chunks = corrupted_body.chunks(wh_block_size as usize);
        let total_source_blocks = chunks.len();

        println!("  scanning {} source blocks...", total_source_blocks);
        let mut bad_blocks = 0;
        
        for (i, chunk) in chunks.enumerate() {
            // VERIFY CRC
            let is_valid = if let Some(expected) = header.compressed_fingerprints.get(i) {
                calculate_checksum(chunk) == *expected
            } else {
                true // Assume valid if no CRC (shouldn't happen)
            };

            if is_valid {
                let _ = decoder.decode(i as u64, chunk, wh_block_size);
            } else {
                bad_blocks += 1;
                // Don't feed. Wirehair treats as erasure.
            }
        }
        println!("  found {} corrupted blocks. Feeding recovery shield...", bad_blocks);

        // Feed Parity Symbols
        for (i, chunk) in recovery_shield.chunks(wh_block_size as usize).enumerate() {
            let id = (total_source_blocks + i) as u64;
            let res = decoder.decode(id, chunk, wh_block_size)
                .map_err(|e| anyhow::anyhow!("Wirehair decode failed: {:?}", e))?;
            
            if res == WirehairResult::Success {
                println!("  ✨ Wirehair success! Sufficient symbols acquired.");
                break;
            }
        }

        let mut repaired_body = vec![0u8; corrupted_body.len()];
        decoder.recover(&mut repaired_body, corrupted_body.len() as u64)
             .map_err(|e| anyhow::anyhow!("Phoenix Protocol Failed: Damage ({}) exceeds parity capacity. {:?}", bad_blocks, e))?;
        
        println!("  ✅ Body repaired. Resuming decompression...");
        
        // Verify repair success
        if !self.check_compressed_integrity(&repaired_body, header) {
             bail!("Phoenix Protocol failed: Repaired body still fails CRC check.");
        }

        self.lzma_decompress(&repaired_body, header.original_size)
    }
}