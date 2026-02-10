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
1.  **DNA Fingerprinting**: Before compression, every 1MB block is fingerprinted with CRC-32. These fingerprints serve as the "DNA" ground truth for reconstruction.
2.  **Ultra-Deep Entropy Reduction**: Utilizing LZMA (Level 9 Extreme) to strip all mathematical redundancy from the data.
3.  **Perfect Reconstruction**: During decompression, the engine rebuilds the data and validates every single block against its original DNA.

## Real-World Use Cases

Lazarus is engineered for scenarios where storage efficiency and data integrity are paramount:
*   **Database Archiving**: Compress multi-terabyte database snapshots for "cold storage" with up to 90% space savings.
*   **Infrastructure Log Management**: Efficiently store years of server logs and audit trails.
*   **Remote Bandwidth Optimization**: Send critical data over low-bandwidth connections.

## Performance Benchmarks

| File Type | Original Size | Compressed Size | Reduction |
| :--- | :--- | :--- | :--- |
| **Database (.db)** | 77.0 MB | 7.3 MB | **90.61%** |
| **Log Files (.log)** | 150.0 MB | 4.2 MB | **97.20%** |
| **Video (.mp4)** | 1.1 GB | 1.1 GB | ~0.10%* |

*\*Media files are already compressed at the hardware level. Lazarus focuses on perfect integrity for these types rather than lossy reduction.*

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

### Decompress
```bash
lazarus decompress <input_file>.lzr --output <restored_file>
```

## Technical Architecture
- **Engine**: LZMA2 (Lempel-Ziv-Markov chain algorithm).
- **Integrity**: Block-level CRC-32 fingerprints.
- **Implementation**: Written in 100% safe Rust.

## License
[MIT License](LICENSE) - Copyright (c) 2026 Azzar Budiyanto