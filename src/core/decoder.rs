use crate::metadata::LazarusHeader;
use crate::core::calculate_checksum;
use anyhow::{Result, bail, Context};
use std::io::Read;
use xz2::read::XzDecoder;

pub struct Decoder;

impl Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decompress(&self, compressed_data: &[u8], header: &LazarusHeader) -> Result<Vec<u8>> {
        if &header.magic != b"LZRS" {
            bail!("Invalid file format: Magic bytes mismatch");
        }

        // 1. LZMA Decompression
        println!("  Decompressing LZMA stream...");
        let mut decompressor = XzDecoder::new(compressed_data);
        let mut reconstructed_data = Vec::with_capacity(header.original_size as usize);
        decompressor.read_to_end(&mut reconstructed_data)
            .context("LZMA decompression failed")?;

        // 2. DNA Verification
        let chunk_size = header.block_size as usize;
        for (i, chunk) in reconstructed_data.chunks(chunk_size).enumerate() {
            let actual_crc = calculate_checksum(chunk);
            if let Some(expected_crc) = header.fingerprints.get(i) {
                if actual_crc != *expected_crc {
                    bail!("Block {} verification failed!", i);
                }
            }
        }

        Ok(reconstructed_data)
    }
}