use crate::metadata::LazarusHeader;
use crate::core::calculate_checksum;
use anyhow::{Result, Context};
use std::io::Read;
use xz2::read::XzEncoder;
use wirehair_wrapper::wirehair::WirehairEncoder;
use rayon::prelude::*;

pub struct Encoder {
    density: f32,
    block_size: Option<u32>,
}

impl Encoder {
    pub fn new(density: f32, block_size: Option<u32>) -> Self {
        Self { density, block_size }
    }

    fn calculate_optimal_block_size(file_size: usize) -> u32 {
        const MIN_BLOCK: u32 = 4096;
        const MAX_BLOCK: u32 = 1_048_576;
        if file_size < 10 * 1024 * 1024 {
            if file_size < 1024 * 1024 { MIN_BLOCK } else { 16384 }
        } else if file_size < 500 * 1024 * 1024 {
            if file_size < 100 * 1024 * 1024 { 65536 } else { 262144 }
        } else {
            MAX_BLOCK
        }
    }

    pub fn compress(&self, input: &[u8], is_folder: bool) -> Result<(Vec<u8>, Vec<u8>, LazarusHeader)> {
        let block_size = self.block_size.unwrap_or_else(|| {
            let optimal = Self::calculate_optimal_block_size(input.len());
            println!("  Adaptive Chunking: Selected {} bytes for {} input bytes.", optimal, input.len());
            optimal
        });

        // 1. DNA Fingerprinting (Original Data) - Parallel
        let fingerprints: Vec<u32> = input.par_chunks(block_size as usize)
            .map(|chunk| calculate_checksum(chunk))
            .collect();

        // 2. Deep LZMA Compression - Parallel per chunk
        println!("  Applying Deep LZMA (Level 9) in parallel...");
        let compressed_chunks: Result<Vec<Vec<u8>>> = input.par_chunks(block_size as usize)
            .map(|chunk| {
                let mut compressor = XzEncoder::new(chunk, 9);
                let mut compressed_chunk = Vec::new();
                compressor.read_to_end(&mut compressed_chunk)
                    .context("LZMA compression failed")?;
                Ok(compressed_chunk)
            })
            .collect();
        
        let compressed_chunks = compressed_chunks?;
        let compressed_chunk_sizes: Vec<usize> = compressed_chunks.iter().map(|c| c.len()).collect();
        let compressed_data: Vec<u8> = compressed_chunks.into_iter().flatten().collect();

        // 3. Generate Wirehair Recovery Symbols
        let wh_block_size = 1024;
        let mut compressed_fingerprints = Vec::new();
        let mut recovery_data = Vec::new();

        if compressed_data.len() >= wh_block_size as usize * 2 {
            println!("  Generating Recovery Shield (5% Parity)...");
            
            // Calculate CRCs for compressed blocks - Parallel
            compressed_fingerprints = compressed_data.par_chunks(wh_block_size as usize)
                .map(|chunk| calculate_checksum(chunk))
                .collect();

            let recovery_overhead = 0.05;
            let recovery_len = (compressed_data.len() as f32 * recovery_overhead).ceil() as usize;
            let wh_encoder = WirehairEncoder::new(&compressed_data, compressed_data.len() as u64, wh_block_size);
            
            let symbols_needed = (recovery_len as f32 / wh_block_size as f32).ceil() as u32;
            let total_source_blocks = (compressed_data.len() as f32 / wh_block_size as f32).ceil() as u64;
            
            for i in 0..symbols_needed {
                let mut block = vec![0u8; wh_block_size as usize];
                let mut out_bytes = 0u32;
                // Important: Ask for blocks STARTING after the source blocks to get Parity!
                let block_id = total_source_blocks + i as u64;
                wh_encoder.encode(block_id, &mut block, wh_block_size, &mut out_bytes)
                    .map_err(|e| anyhow::anyhow!("Wirehair encoding failed: {:?}", e))?;
                recovery_data.extend_from_slice(&block);
            }
        } else {
            println!("  Note: Input too small for recovery shield. Skipping.");
        }

        let mut header = LazarusHeader {
            magic: *b"LZRS",
            original_size: input.len() as u64,
            block_size,
            total_blocks: fingerprints.len() as u32,
            density: self.density,
            fingerprints,
            has_recovery: !recovery_data.is_empty(),
            recovery_len: recovery_data.len() as u64,
            compressed_fingerprints,
            is_folder,
            compressed_chunk_sizes,
            header_checksum: 0,
        };

        // 4. Final Header DNA (Structural Integrity)
        let header_bytes = bincode::serialize(&header)?;
        let header_checksum = calculate_checksum(&header_bytes);
        header.header_checksum = header_checksum;

        Ok((compressed_data, recovery_data, header))
    }
}
