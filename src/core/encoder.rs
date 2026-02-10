use crate::metadata::LazarusHeader;
use crate::core::calculate_checksum;
use anyhow::{Result, Context};
use std::io::Read;
use xz2::read::XzEncoder;
use xz2::read::XzDecoder;

pub struct Encoder {
    density: f32,
    block_size: u32,
}

impl Encoder {
    pub fn new(density: f32, block_size: u32) -> Self {
        Self { density, block_size }
    }

    pub fn compress(&self, input: &[u8]) -> Result<(Vec<u8>, LazarusHeader)> {
        let mut fingerprints = Vec::new();
        for chunk in input.chunks(self.block_size as usize) {
            fingerprints.push(calculate_checksum(chunk));
        }

        // Layer 1: Deep LZMA (Compression Level 9 - Extreme)
        println!("  Applying Deep LZMA (Level 9)...");
        let mut compressor = XzEncoder::new(input, 9);
        let mut compressed_data = Vec::new();
        compressor.read_to_end(&mut compressed_data)
            .context("LZMA compression failed")?;

        let header = LazarusHeader {
            magic: *b"LZRS",
            original_size: input.len() as u64,
            block_size: self.block_size,
            total_blocks: fingerprints.len() as u32,
            density: self.density,
            fingerprints,
        };

        Ok((compressed_data, header))
    }
}