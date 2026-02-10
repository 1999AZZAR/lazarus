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
#[command(about = "High-density compression using Wirehair and CRC-32", long_about = None)]
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
        /// Output file path (optional, defaults to input.lzr)
        #[arg(short, long)]
        output: Option<String>,
        /// Compression density (0.1 - 0.9). Currently correlates to Zstd level/effort.
        #[arg(short, long, default_value_t = 0.5)]
        density: f32,
    },
    /// Decompress a file
    Decompress {
        /// Input .lzr file path
        input: String,
        /// Output file path (optional, defaults to original name if possible or input.out)
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<()> {
    wirehair_wrapper::wirehair::wirehair_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize Wirehair: {:?}", e))?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { input, output, density } => {
            let input_path = Path::new(&input);
            let output_path = output.unwrap_or_else(|| format!("{}.lzr", input));

            println!("Reading {}...", input);
            let data = fs::read(input_path).context("Failed to read input file")?;
            let original_size = data.len();

            println!("Encoding chunks (Block Size: 1MB)...");
            let encoder = Encoder::new(density, 1048576); 
            let (compressed_data, header) = encoder.compress(&data)?;

            // Serialize Header
            let header_bytes = bincode::serialize(&header)?;
            let header_len = header_bytes.len() as u32;

            let mut out_file = File::create(&output_path)?;
            
            // Write Header Length (4 bytes) + Header + Body
            out_file.write_all(&header_len.to_le_bytes())?;
            out_file.write_all(&header_bytes)?;
            out_file.write_all(&compressed_data)?;

            let final_size = out_file.metadata()?.len();
            let ratio = 1.0 - (final_size as f64 / original_size as f64);

            println!("Success! Saved to {}", output_path);
            println!("Original: {} bytes", original_size);
            println!("Lazarus:  {} bytes", final_size);
            println!("Reduction: {:.2}%", ratio * 100.0);
            
            if ratio < 0.4 {
                println!("Warning: 40% target not met. Data may be high entropy already.");
            }
        }
        Commands::Decompress { input, output } => {
            println!("Reading {}...", input);
            let mut file = File::open(&input)?;
            
            // Read Header Length
            let mut len_buf = [0u8; 4];
            file.read_exact(&mut len_buf).context("Failed to read header length")?;
            let header_len = u32::from_le_bytes(len_buf) as usize;

            // Read Header
            let mut header_buf = vec![0u8; header_len];
            file.read_exact(&mut header_buf).context("Failed to read header")?;
            let header: LazarusHeader = bincode::deserialize(&header_buf)?;

            // Read Body
            let mut body_buf = Vec::new();
            file.read_to_end(&mut body_buf)?;

            println!("Reconstructing {} blocks...", header.total_blocks);
            let decoder = Decoder::new();
            let reconstructed = decoder.decompress(&body_buf, &header)?;

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
