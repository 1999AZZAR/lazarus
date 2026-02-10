# Lazarus: High-Density "DNA" Compression

Lazarus is a high-density compression engine designed for extreme storage optimization and perfect data reconstruction. It uses a multi-layered approach combining deep entropy reduction with block-level integrity fingerprints to ensure that even the most aggressive compression remains 100% reliable.

## Table of Contents

- [Why Lazarus?](#why-lazarus)
- [The Core Philosophy](#the-core-philosophy)
- [Real-World Use Cases](#real-world-use-cases)
- [Performance Benchmarks](#performance-benchmarks)
- [Installation](#installation)
  - [Debian/Ubuntu (.deb)](#debianubuntu-deb)
  - [From Source](#from-source)
- [Usage](#usage)
- [Technical Architecture](#technical-architecture)
- [License](#license)

## Why "Lazarus"?

The name **Lazarus** is inspired by the concept of miraculous restoration. In our engine, data is intentionally "stripped down" to its absolute minimum—discarding over 90% of its physical footprint in many cases. However, because we preserve the **"DNA"** (CRC-32 fingerprints) of every block, the original data can be "resurrected" from its compressed state with absolute bit-for-bit perfection. It represents the bridge between extreme data loss (via compression) and total recovery.

## The Core Philosophy

Traditional compression often stops at standard entropy limits. Lazarus pushes further by:
1.  **Adaptive Chunking**: Automatically scales block sizes (4KB to 1MB) based on input file size to balance metadata overhead and reconstruction granularity.
2.  **Phoenix Protocol (Self-Healing)**: Embeds a 5% Wirehair parity shield into the archive. If disk rot or corruption occurs, Lazarus can mathematically reconstruct the broken parts of the file using Fountain Codes.
3.  **DNA Fingerprinting**: Before compression, every block is fingerprinted with CRC-32. These fingerprints serve as the "DNA" ground truth for reconstruction.
4.  **Ultra-Deep Entropy Reduction**: Utilizing LZMA (Level 9 Extreme) to strip all mathematical redundancy from the data.
5.  **Perfect Reconstruction**: During decompression, the engine rebuilds the data and validates every single block against its original DNA.

## Real-World Use Cases

Lazarus is engineered for scenarios where storage efficiency and data integrity are paramount:
*   **Database Archiving**: Compress multi-terabyte database snapshots for "cold storage" with up to 90% space savings.
*   **Infrastructure Log Management**: Efficiently store years of server logs and audit trails.
*   **Remote Bandwidth Optimization**: Send critical data over low-bandwidth connections.

## Performance Benchmarks

Tests were conducted on a Linux x86_64 environment using synthetic datasets.

| File Type | Original Size | Compressed Size | Reduction | Compression Time | Decompression Time |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Server Logs (.log)** | 100 MB | 7.6 MB | **92.46%** | 38.59s | 0.50s |
| **JSON Data (.json)** | 50 MB | 6.0 MB | **88.03%** | 17.45s | 0.33s |
| **Database (.db)** | 77.0 MB | 7.3 MB | **90.61%** | 18.20s | 0.60s |
| **Binary/Media** | 1.1 GB | 1.1 GB | ~0.10%* | N/A | N/A |

*\*Note: Already-compressed media files (MP4, JPG) see minimal reduction but benefit from Lazarus's block-level integrity verification.*


## Installation

### Debian/Ubuntu (.deb)
1. Download the latest `.deb` package from the [Releases](https://github.com/1999AZZAR/lazarus/releases) page.
2. Install it using `dpkg`:
```bash
sudo dpkg -i lazarus_0.1.0_amd64.deb
```

### From Source
Ensure you have the Rust toolchain installed.
```bash
cargo build --release
sudo cp target/release/lazarus /usr/bin/
```

## Usage

Once installed, `lazarus` is available as a global command.

### Compress
```bash
lazarus compress <input_file> --output <output_file>.lzr
```
*Note: Lazarus automatically selects the optimal block size, but you can override it using `--block-size <bytes>`.*

### Decompress
```bash
lazarus decompress <input_file>.lzr --output <restored_file>
```

## Technical Architecture
- **Engine**: LZMA2 (Lempel-Ziv-Markov chain algorithm).
- **Integrity**: Block-level CRC-32 fingerprints.
- **Efficiency**: Adaptive Block Sizing (4KB - 1MB range).
- **Implementation**: Written in 100% safe Rust.

## License
[MIT License](LICENSE) - Copyright (c) 2026 Azzar Budiyanto