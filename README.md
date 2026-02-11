# Lazarus: High-Density DNA Compression Engine

Lazarus is a specialized compression engine written in Rust, engineered for long-term data preservation and secure archival. It integrates deep entropy reduction (LZMA) with block-level integrity fingerprints (CRC-32), cryptographic confidentiality (ChaCha20-Poly1305), and mathematical self-healing (Wirehair Fountain Codes).

The primary objective of Lazarus is to ensure that archived data remains retrievable and private even when stored on unstable media or subjected to partial bit-rot and physical corruption.

## Technical Architecture

Lazarus utilizes a multi-layered approach to data protection:

1.  **Adaptive Block Chunking**: Input data is partitioned into dynamic blocks (ranging from 4KB to 1MB) based on total file size. This strategy balances metadata overhead against the granularity of potential data recovery.
2.  **DNA Fingerprinting**: Every block undergoes CRC-32 verification before and after the compression/encryption stages. These fingerprints serve as the authoritative ground truth during the reconstruction process.
3.  **Entropy Reduction**: Data is compressed using LZMA (Level 9) to achieve maximum theoretical density.
4.  **Secret Shield (Confidentiality)**: When enabled, compressed blocks are encrypted using the ChaCha20-Poly1305 AEAD algorithm. Encryption keys are derived from user passwords using the Argon2id key derivation function (KDF).
5.  **Phoenix Protocol (Self-Healing)**: Lazarus applies Wirehair Fountain Codes to generate an adaptive parity shield (3% to 10% overhead). This allows for bit-perfect reconstruction of corrupted blocks without requiring original source data.
6.  **Structural Redundancy**: The archive utilizes a dual-header "Brain Backup" strategy. If the primary header fails integrity checks, the system automatically resurrects the archive structure from a redundant metadata block.

## Comparison vs Standard Tools

The following table summarizes the capabilities of Lazarus v0.1.7 compared to industry-standard archival tools.

| Feature | Lazarus | XZ / 7-Zip | Gzip / Zip |
| :--- | :--- | :--- | :--- |
| **Primary Algorithm** | LZMA (L9) | LZMA2 | DEFLATE |
| **Integrity Check** | Block-Level (DNA) | Stream-Level | File-Level |
| **Confidentiality** | ChaCha20-Poly1305 | AES-256 | Mixed |
| **Self-Healing** | Built-in (Phoenix) | None | None |
| **Parallelism** | Rayon (Multithreaded) | Semi-Supported | Limited |
| **Data Resurrection** | Mathematical | External Parity Only | None |
| **Architecture** | x86_64 / ARM64 | Universal | Universal |

## Performance Benchmarks

Recent testing conducted on an x86_64 environment using Lazarus v0.1.7.

### Compression Efficiency and Resilience
Tests were conducted by injecting 1KB of random data (corruption) into the archive data stream.

| File Type | Original Size | Lazarus Size | Reduction | Chaos Resilience |
| :--- | :--- | :--- | :--- | :--- |
| **Server Logs** | 50 MB | 196 KB | 99.63% | **SUCCESS (Healed)** |
| **JSON Data** | 50 MB | 2.2 MB | 95.48% | **SUCCESS (Healed)** |
| **Binary Data**| 10 MB | 10.6 MB | -5.93% | **SUCCESS (Healed)** |

*Note: Binary data archives include a higher adaptive parity overhead (10%) to ensure recovery for high-entropy payloads.*

## Installation

### From Source
Ensure the Rust toolchain is installed, then execute:
```bash
cargo build --release
sudo cp target/release/lazarus /usr/bin/
```

### Debian/Ubuntu (.deb)
Official packages are available on the GitHub Releases page:
```bash
sudo dpkg -i lazarus_0.1.7_amd64.deb
```

## Usage

### Compression
To compress a file or an entire directory:
```bash
lazarus compress <path>
```

To enable encryption (Secret Shield):
```bash
lazarus compress <path> --password "your-secure-phrase"
```

### Decompression
To decompress and verify an archive:
```bash
lazarus decompress <file.lzr>
```

For encrypted archives:
```bash
lazarus decompress <file.lzr> --password "your-secure-phrase"
```

## Implementation Details

- **Concurrency**: Lazarus leverages the `rayon` library for parallel processing of compression and encryption blocks, significantly reducing wall-clock time on multi-core systems.
- **Key Derivation**: Argon2id is utilized for password-to-key conversion, providing high resistance against GPU-based brute-force attacks.
- **Error Correction**: The `wirehair-wrapper` crate provides the underlying fountain code logic for data resurrection.

## Documentation

Comprehensive technical documentation, implementation reports, and detailed performance analysis can be found in the [docs/](docs/) directory.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details. Copyright (c) 2026 Azzar Budiyanto.
