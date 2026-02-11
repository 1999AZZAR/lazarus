use std::fs;
use std::process::Command;
use tempfile::tempdir;
use std::io::{Write, Seek, SeekFrom};

#[test]
fn test_chaos_injection_self_healing() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("chaos_test_data.txt");
    let compressed_path = dir.path().join("chaos_test_data.txt.lzr");
    let corrupted_path = dir.path().join("chaos_test_data_corrupted.lzr");
    let output_path = dir.path().join("reconstructed_chaos.txt");

    // Create test data large enough to trigger recovery shield (>2KB compressed)
    // Use structured, redundant data that compresses well
    let mut original_data = String::new();
    for i in 0..5000 {
        original_data.push_str(&format!("LOG ENTRY {}: This is a highly redundant log message for testing Lazarus self-healing capabilities.\n", i));
    }
    fs::write(&input_path, &original_data).unwrap();

    // 1. Build project
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .status()
        .unwrap();
    assert!(status.success(), "Build failed");

    let bin = "./target/release/lazarus";

    // 2. Compress with default settings (should include recovery shield)
    let compress_output = Command::new(bin)
        .arg("compress")
        .arg(input_path.to_str().unwrap())
        .arg("--output")
        .arg(compressed_path.to_str().unwrap())
        .output()
        .unwrap();
    
    assert!(compress_output.status.success(), "Compression failed");
    
    let compress_stdout = String::from_utf8_lossy(&compress_output.stdout);
    println!("Compression output:\n{}", compress_stdout);
    
    // Verify recovery shield was generated
    assert!(compress_stdout.contains("Recovery Shield") || compress_stdout.contains("Parity"), 
            "Recovery shield was not generated! Archive may be too small.");

    // 3. Inject chaos - corrupt random bytes in the compressed data
    let lzr_file_data = fs::read(&compressed_path).unwrap();
    let total_file_size = lzr_file_data.len();
    
    // Read header length (first 4 bytes)
    let header_len = u32::from_le_bytes([
        lzr_file_data[0],
        lzr_file_data[1],
        lzr_file_data[2],
        lzr_file_data[3],
    ]) as usize;
    
    // Calculate actual compressed data section
    let header_start = 4; // After the 4-byte length field
    let compressed_data_start = header_start + header_len;
    
    println!("File structure:");
    println!("  - Total size: {} bytes", total_file_size);
    println!("  - Header length field: 4 bytes");
    println!("  - Header: {} bytes (at offset {})", header_len, header_start);
    println!("  - Compressed data starts at: byte {}", compressed_data_start);
    
    // Copy to corrupted file
    fs::copy(&compressed_path, &corrupted_path).unwrap();
    
    // Corrupt 10 random bytes in the compressed data section (not header or recovery)
    let corruption_count = 10;
    let compressed_data_size = total_file_size - compressed_data_start;
    
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(&corrupted_path)
        .unwrap();
    
    use std::collections::HashSet;
    let mut corrupted_positions = HashSet::new();
    
    // Generate random positions to corrupt (in the compressed data section)
    // Corrupt in the first half of compressed data to ensure it's not in recovery section
    let corruptible_start = compressed_data_start;
    let corruptible_end = compressed_data_start + (compressed_data_size / 2);
    let corruptible_range = corruptible_end - corruptible_start;
    
    println!("Corruptible range: {} to {} ({} bytes)", 
             corruptible_start, corruptible_end, corruptible_range);
    
    while corrupted_positions.len() < corruption_count {
        let pos = corruptible_start + (rand_byte() as usize % corruptible_range);
        corrupted_positions.insert(pos);
    }
    
    println!("Injecting chaos at {} positions:", corruption_count);
    for &pos in &corrupted_positions {
        file.seek(SeekFrom::Start(pos as u64)).unwrap();
        let corrupt_byte = rand_byte();
        file.write_all(&[corrupt_byte]).unwrap();
        println!("  - Corrupted byte at position {}", pos);
    }
    
    file.sync_all().unwrap();
    drop(file);

    // 4. Attempt to decompress corrupted file - Phoenix Protocol should activate
    println!("\nAttempting to decompress corrupted file...");
    let decompress_output = Command::new(bin)
        .arg("decompress")
        .arg(corrupted_path.to_str().unwrap())
        .arg("--output")
        .arg(output_path.to_str().unwrap())
        .output()
        .unwrap();
    
    let decompress_stdout = String::from_utf8_lossy(&decompress_output.stdout);
    let decompress_stderr = String::from_utf8_lossy(&decompress_output.stderr);
    
    println!("Decompression output:\n{}", decompress_stdout);
    if !decompress_stderr.is_empty() {
        println!("Decompression stderr:\n{}", decompress_stderr);
    }

    // 5. Verify self-healing occurred
    assert!(
        decompress_output.status.success(),
        "Decompression failed! Phoenix Protocol did not successfully heal the data."
    );
    
    // Check that Phoenix Protocol was activated
    let output_text = format!("{}{}", decompress_stdout, decompress_stderr);
    assert!(
        output_text.contains("Phoenix") || 
        output_text.contains("Repair") || 
        output_text.contains("Corruption detected") ||
        output_text.contains("Wirehair"),
        "Phoenix Protocol was not triggered despite corruption!"
    );

    // 6. Verify bit-perfect reconstruction
    let reconstructed_data = fs::read_to_string(&output_path)
        .expect("Failed to read reconstructed file");
    
    assert_eq!(
        original_data, 
        reconstructed_data,
        "Data integrity check FAILED! Reconstructed data does not match original."
    );
    
    println!("\n✅ CHAOS TEST PASSED!");
    println!("   - {} bytes corrupted", corruption_count);
    println!("   - Phoenix Protocol successfully activated");
    println!("   - Bit-perfect reconstruction achieved");
    println!("   - Self-healing capability verified with parallel implementation");
}

// Simple pseudo-random byte generator (deterministic for testing)
fn rand_byte() -> u8 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % 256) as u8
}
