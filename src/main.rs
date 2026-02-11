mod core;
mod io;
mod metadata;

use clap::{Parser, Subcommand};
use anyhow::{Result, Context};
use std::fs::{self, File};
use std::io::{Write, Read, Cursor, Seek};
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
            
            // Sanitize default output name
            let output_path = output.unwrap_or_else(|| {
                let base_name = input_path.file_name()
                    .and_then(|s| Some(s.to_string_lossy().into_owned()))
                    .filter(|s| s != "." && s != "..")
                    .unwrap_or_else(|| {
                        // Fallback to current directory name if input is "." or similar
                        std::env::current_dir().ok()
                            .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
                            .unwrap_or_else(|| "archive".to_string())
                    });
                format!("{}.lzr", base_name)
            });

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

            // Serialize Header with calculated DNA
            let header_bytes = bincode::serialize(&header)?;
            let header_len = header_bytes.len() as u32;

            let mut out_file = File::create(&output_path)?;
            
            // Layout: [Len:4][Header:N][HeaderCopy:N][Compressed:M][Recovery:K]
            out_file.write_all(&header_len.to_le_bytes())?;
            out_file.write_all(&header_bytes)?;
            out_file.write_all(&header_bytes)?; // Brain Redundancy (Backup copy of the header)
            out_file.write_all(&compressed_data)?;
            out_file.write_all(&recovery_data)?;

            let final_size = out_file.metadata()?.len();
            let ratio = 1.0 - (final_size as f64 / original_size as f64);

            println!("Success! Saved to {}", output_path);
            println!("Original: {} bytes", original_size);
            println!("Lazarus:  {} bytes (Inc. Brain Redundancy & Recovery Shield)", final_size);
            println!("Reduction: {:.2}%", ratio * 100.0);
        },
        Commands::Decompress { input, output } => {
            println!("Reading {}...", input);
            let mut file = File::open(&input)?;
            let total_len = file.metadata()?.len();
            
            let mut len_buf = [0u8; 4];
            file.read_exact(&mut len_buf).context("Failed to read header length")?;
            let header_len = u32::from_le_bytes(len_buf) as usize;

            // Attempt to load Primary Brain
            let mut header_buf = vec![0u8; header_len];
            file.read_exact(&mut header_buf).context("Failed to read primary header")?;
            
            let header: Result<LazarusHeader> = (|| {
                let h: LazarusHeader = bincode::deserialize(&header_buf)?;
                // Verify DNA of Primary Brain
                let mut check_h = h.clone();
                check_h.header_checksum = 0;
                let actual_sum = crate::core::calculate_checksum(&bincode::serialize(&check_h)?);
                if actual_sum != h.header_checksum {
                    anyhow::bail!("Primary header DNA mismatch");
                }
                Ok(h)
            })();

            let header = match header {
                Ok(h) => {
                    // Skip the backup copy since primary is fine
                    file.seek(std::io::SeekFrom::Current(header_len as i64))?;
                    h
                },
                Err(e) => {
                    println!("  Warning: Primary Brain corrupted ({}). Attempting Resurrection from Backup...", e);
                    let mut backup_buf = vec![0u8; header_len];
                    file.read_exact(&mut backup_buf).context("Failed to read backup header")?;
                    let h: LazarusHeader = bincode::deserialize(&backup_buf)
                        .context("Backup header also corrupted. Data loss is irreversible.")?;
                    
                    // Verify DNA of Backup Brain
                    let mut check_h = h.clone();
                    check_h.header_checksum = 0;
                    let actual_sum = crate::core::calculate_checksum(&bincode::serialize(&check_h)?);
                    if actual_sum != h.header_checksum {
                        anyhow::bail!("Backup header DNA mismatch. Archive is fundamentally broken.");
                    }
                    println!("  Success: Header resurrected from redundant backup.");
                    h
                }
            };

            // Calculate start of payload based on redundant header layout
            let payload_start = 4 + (header_len as u64 * 2);
            let compressed_len = total_len - payload_start - header.recovery_len;
            
            file.set_len(total_len)?; // Ensure seek is valid
            file.seek(std::io::SeekFrom::Start(payload_start))?;

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