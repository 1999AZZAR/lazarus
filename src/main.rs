mod core;
mod io;
mod metadata;

use clap::{Parser, Subcommand};
use anyhow::{Result, Context, bail};
use std::fs::{self, File};
use std::io::{Write, Read, Cursor};
use std::path::Path;
use crate::core::encoder::Encoder;
use crate::core::decoder::Decoder;
use crate::metadata::LazarusHeader;
use tar::Builder;

#[derive(Parser)]
#[command(name = "lazarus")]
#[command(about = "High-density compression with Folder Support and Self-Healing", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a file or folder
    Compress {
        /// Input file or folder path
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
        /// Output path (file or folder)
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

            let (data, is_folder) = if input_path.is_dir() {
                println!("Bundling folder {}...", input);
                let mut tar_builder = Builder::new(Vec::new());
                tar_builder.append_dir_all(".", input_path)?;
                let tar_data = tar_builder.into_inner()?;
                (tar_data, true)
            } else {
                println!("Reading file {}...", input);
                (fs::read(input_path).context("Failed to read input file")?, false)
            };

            let original_size = data.len();
            let encoder = Encoder::new(density, block_size); 
            let (compressed_data, recovery_data, header) = encoder.compress(&data, is_folder)?;

            // Serialize Header
            let header_bytes = bincode::serialize(&header)?;
            let header_len = header_bytes.len() as u32;

            let mut out_file = File::create(&output_path)?;
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
            
            let mut len_buf = [0u8; 4];
            file.read_exact(&mut len_buf).context("Failed to read header length")?;
            let header_len = u32::from_le_bytes(len_buf) as usize;

            let mut header_buf = vec![0u8; header_len];
            file.read_exact(&mut header_buf).context("Failed to read header")?;
            let header: LazarusHeader = bincode::deserialize(&header_buf)?;

            let compressed_len = total_len - 4 - header_len as u64 - header.recovery_len;
            let mut compressed_buf = vec![0u8; compressed_len as usize];
            file.read_exact(&mut compressed_buf).context("Failed to read compressed body")?;
            
            let mut recovery_buf = Vec::new();
            if header.has_recovery {
                file.read_to_end(&mut recovery_buf)?;
            }

            println!("Reconstructing...");
            let decoder = Decoder::new();
            let reconstructed = decoder.decompress(&compressed_buf, &recovery_buf, &header)?;

            let output_path = output.unwrap_or_else(|| {
                let path_str = input.trim_end_matches(".lzr");
                if path_str == input {
                    format!("{}.out", input)
                } else {
                    path_str.to_string()
                }
            });

            if header.is_folder {
                println!("Extracting to folder {}...", output_path);
                fs::create_dir_all(&output_path)?;
                let mut archive = tar::Archive::new(Cursor::new(reconstructed));
                archive.unpack(&output_path)?;
            } else {
                fs::write(&output_path, reconstructed)?;
            }
            
            println!("Success! Reconstructed to {}", output_path);
        }
    }

    Ok(())
}