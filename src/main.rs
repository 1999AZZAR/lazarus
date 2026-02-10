mod core;
mod io;
mod metadata;

use clap::{Parser, Subcommand};
use anyhow::{Result, Context};
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;
use crate::core::encoder::Encoder;
use crate::core::decoder::Decoder;
use crate::metadata::LazarusHeader;

#[derive(Parser)]
#[command(name = "lazarus")]
#[command(about = "High-density compression with Self-Healing Recovery", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a file
    Compress {
        /// Input file path
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Compression density (Reserved).
        #[arg(short, long, default_value_t = 0.5)]
        density: f32,
        /// Block size in bytes.
        #[arg(short, long)]
        block_size: Option<u32>,
    },
    /// Decompress a file
    Decompress {
        /// Input .lzr file path
        input: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<()> {
    wirehair_wrapper::wirehair::wirehair_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize Wirehair: {:?}", e))?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { input, output, density, block_size } => {
            let input_path = Path::new(&input);
            let output_path = output.unwrap_or_else(|| format!("{}.lzr", input));

            println!("Reading {}...", input);
            let data = fs::read(input_path).context("Failed to read input file")?;
            let original_size = data.len();

            let encoder = Encoder::new(density, block_size); 
            // Note: compress() now returns (compressed, recovery, header)
            let (compressed_data, recovery_data, header) = encoder.compress(&data)?;

            // Serialize Header
            let header_bytes = bincode::serialize(&header)?;
            let header_len = header_bytes.len() as u32;

            let mut out_file = File::create(&output_path)?;
            
            // Format: [Len: 4][Header][Compressed Body][Recovery Data]
            out_file.write_all(&header_len.to_le_bytes())?;
            out_file.write_all(&header_bytes)?;
            out_file.write_all(&compressed_data)?;
            out_file.write_all(&recovery_data)?;

            let final_size = out_file.metadata()?.len();
            let ratio = 1.0 - (final_size as f64 / original_size as f64);

            println!("Success! Saved to {}", output_path);
            println!("Original: {} bytes", original_size);
            println!("Lazarus:  {} bytes (Inc. Recovery Shield)", final_size);
            println!("Reduction: {:.2}%", ratio * 100.0);
        },
        Commands::Decompress { input, output } => {
            println!("Reading {}...", input);
            let mut file = File::open(&input)?;
            let total_len = file.metadata()?.len();
            
            // Read Header Length
            let mut len_buf = [0u8; 4];
            file.read_exact(&mut len_buf).context("Failed to read header length")?;
            let header_len = u32::from_le_bytes(len_buf) as usize;

            // Read Header
            let mut header_buf = vec![0u8; header_len];
            file.read_exact(&mut header_buf).context("Failed to read header")?;
            let header: LazarusHeader = bincode::deserialize(&header_buf)?;

            // Determine sizes
            let compressed_len = total_len - 4 - header_len as u64 - header.recovery_len;
            
            // Read Compressed Body
            let mut compressed_buf = vec![0u8; compressed_len as usize];
            file.read_exact(&mut compressed_buf).context("Failed to read compressed body")?;
            
            // Read Recovery Data (if present)
            let mut recovery_buf = Vec::new();
            if header.has_recovery {
                file.read_to_end(&mut recovery_buf)?;
            }

            println!("Reconstructing {} blocks...", header.total_blocks);
            let decoder = Decoder::new();
            // Pass recovery buffer to decoder
            let reconstructed = decoder.decompress(&compressed_buf, &recovery_buf, &header)?;

            let output_path = output.unwrap_or_else(|| {
                let path_str = input.trim_end_matches(".lzr");
                if path_str == input {
                    format!("{}.out", input)
                } else {
                    path_str.to_string()
                }
            });

            fs::write(&output_path, reconstructed)?;
            println!("Success! Reconstructed to {}", output_path);
        }
    }

    Ok(())
}
