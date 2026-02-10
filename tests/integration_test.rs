use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_lazarus_cycle() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("test_data.txt");
    let compressed_path = dir.path().join("test_data.txt.lzr");
    let output_path = dir.path().join("reconstructed.txt");

    // Create high-redundancy data (should compress > 40%)
    let original_data = "Lazarus ".repeat(10000); // ~80KB
    fs::write(&input_path, &original_data).unwrap();

    // 1. Build project
    let status = Command::new("cargo")
        .args(&["build"])
        .status()
        .unwrap();
    assert!(status.success());

    let bin = "./target/debug/lazarus";

    // 2. Compress with 0.4 density (40% reduction)
    let status = Command::new(bin)
        .arg("compress")
        .arg(input_path.to_str().unwrap())
        .arg("--output")
        .arg(compressed_path.to_str().unwrap())
        .arg("--density")
        .arg("0.4")
        .status()
        .unwrap();
    assert!(status.success());

    // Check size
    let original_len = fs::metadata(&input_path).unwrap().len();
    let compressed_len = fs::metadata(&compressed_path).unwrap().len();
    println!("Original: {}, Compressed: {}", original_len, compressed_len);
    
    // Ensure ~40% reduction (accounting for header overhead)
    let ratio = compressed_len as f64 / original_len as f64;
    assert!(ratio < 0.65);

    // 3. Decompress
    let status = Command::new(bin)
        .arg("decompress")
        .arg(compressed_path.to_str().unwrap())
        .arg("--output")
        .arg(output_path.to_str().unwrap())
        .status()
        .unwrap();
    assert!(status.success());

    // 4. Verify Content
    let reconstructed_data = fs::read_to_string(&output_path).unwrap();
    assert_eq!(original_data, reconstructed_data);
}
