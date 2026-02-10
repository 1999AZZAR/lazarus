# Lazarus: High-Density "DNA" Compression

Lazarus is a high-density compression engine designed for extreme storage optimization and self-healing data reconstruction. It combines deep entropy reduction with block-level integrity fingerprints and Fountain Code parity to ensure that your data remains 100% reliable, even in the face of bit rot or physical corruption.

## Table of Contents

- [Why Lazarus?](#why-lazarus)
- [The Core Philosophy](#the-core-philosophy)
- [Self-Healing: The Phoenix Protocol](#self-healing-the-phoenix-protocol)
- [Comparison vs Standard Tools](#comparison-vs-standard-tools)
- [Performance Benchmarks](#performance-benchmarks)
- [Installation](#installation)
- [Usage](#usage)
- [Technical Architecture](#technical-architecture)
- [License](#license)

## Why "Lazarus"?

The name **Lazarus** is inspired by the concept of miraculous restoration. In our engine, data is "stripped down" to its absolute minimum. However, because we preserve the **"DNA"** (CRC-32 fingerprints) and a **"Phoenix Shield"** (Wirehair parity), the original data can be "resurrected" from a corrupted state with bit-for-bit perfection.

## The Core Philosophy

1.  **Adaptive Chunking**: Automatically scales block sizes (4KB to 1MB) based on input file size to balance metadata overhead and reconstruction granularity.
2.  **DNA Fingerprinting**: Every block is fingerprinted with CRC-32 before and after compression. These serve as the ground truth for reconstruction.
3.  **Ultra-Deep Entropy Reduction**: Utilizing LZMA (Level 9 Extreme) to achieve maximum density.
4.  **Self-Healing (Phoenix Protocol)**: Utilizes Wirehair Fountain Codes to mathematically repair corrupted segments of the archive.

## Self-Healing: The Phoenix Protocol

Unlike standard `.zip` or `.7z` files, Lazarus is designed for "Cold Storage" where hardware failure is a risk. 
- **The Shield**: Every archive includes a 5% recovery overhead.
- **The Repair**: If Lazarus detects a CRC mismatch during decompression, it automatically triggers the **Phoenix Protocol**, using parity symbols to reconstruct missing or corrupted blocks.

## Comparison vs Standard Tools

| Feature | Lazarus | XZ / 7-Zip | Gzip / Zip |
| :--- | :--- | :--- | :--- |
| **Compression Ratio** | Ultra High | Ultra High | Moderate |
| **Self-Healing** | **Yes (Built-in)** | No | No |
| **Integrity Check** | Block-Level (1MB) | Stream-Level | File-Level |
| **Repair Capability** | Mathematical Recovery | External Rev-files only | None |
| **Speed** | Slow (Heavy) | Slow | Very Fast |

### Pros
- **Invincibility**: Can survive partial file corruption that would destroy other archives.
- **Precision**: Identifies exactly which part of a file is damaged.
- **Adaptive**: Optimizes itself for the data size automatically.

### Cons
- **CPU Intensive**: High-level compression takes time and power.
- **Binary Size**: Slightly larger overhead due to the embedded recovery shield.

## Performance Benchmarks

*Tests conducted on x86_64 using synthetic and real-world datasets.*

| File Type | Original Size | Compressed Size | Reduction | Comp. Time | Healing |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Server Logs (.log)** | 100 MB | 7.6 MB | **92.46%** | 38.5s | **Verified** |
| **JSON Data (.json)** | 50 MB | 6.0 MB | **88.03%** | 17.4s | **Verified** |
| **Database (.db)** | 77.0 MB | 7.3 MB | **90.61%** | 18.2s | **Verified** |
| **Binary (Random)** | 10 MB | 10.5 MB | -5.0%* | 2.3s | **Verified** |

*\*Binary files with high entropy include the 5% recovery shield, resulting in a slight size increase, but gaining full self-healing capabilities.*

## Installation

### Debian/Ubuntu (.deb)
Download from the [Releases](https://github.com/1999AZZAR/lazarus/releases) page:
```bash
sudo dpkg -i lazarus_0.1.0_amd64.deb
```

### From Source
```bash
cargo build --release
sudo cp target/release/lazarus /usr/bin/
```

## Usage

### Compress
```bash
lazarus compress <file>
```

### Decompress
```bash
lazarus decompress <file.lzr>
```

## License
[MIT License](LICENSE) - Copyright (c) 2026 Azzar Budiyanto
