# Lazarus: High-Density DNA Compression

Lazarus is a high-density compression engine designed for extreme storage optimization and self-healing data reconstruction. It combines deep entropy reduction with block-level integrity fingerprints, redundant header backups, and adaptive Fountain Code parity to ensure that your data remains 100% reliable, even in the face of bit rot or physical corruption.

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

The name **Lazarus** is inspired by the concept of miraculous restoration. In our engine, data is "stripped down" to its absolute minimum. However, because we preserve the **"DNA"** (CRC-32 fingerprints), a **"Brain Backup"** (Redundant Headers), and a **"Phoenix Shield"** (Adaptive Parity), the original data can be "resurrected" from a corrupted state with bit-for-bit perfection.

## The Core Philosophy

1.  **Adaptive Chunking**: Automatically scales block sizes (4KB to 1MB) based on input file size to balance metadata overhead and reconstruction granularity.
2.  **DNA Fingerprinting**: Every block is fingerprinted with CRC-32 before and after compression. These serve as the ground truth for reconstruction.
3.  **Ultra-Deep Entropy Reduction**: Utilizing LZMA (Level 9 Extreme) to achieve maximum density.
4.  **Self-Healing (Phoenix Protocol)**: Utilizes Wirehair Fountain Codes to mathematically repair corrupted segments of the archive.
5.  **Brain Redundancy**: Dual-header strategy (Primary + Backup) ensures structural metadata survives even if the start of the archive is physically damaged.

## Self-Healing: The Phoenix Protocol

Unlike standard `.zip` or `.7z` files, Lazarus is designed for "Cold Storage" where hardware failure is a risk. 

- **The Shield**: Every archive includes an **Adaptive Parity** overhead (3% to 10%) optimized for the file size.
- **The Repair**: If Lazarus detects a CRC mismatch, it uses Fountain Code parity symbols to mathematically reconstruct the missing or corrupted data blocks.
- **The Resurrection**: If the archive header is damaged, the system automatically detects the corruption via DNA mismatch and restores the structural "brain" from the redundant backup copy.

### Chaos Resilience Summary
In a direct "Chaos Injection" test comparing Lazarus vs industry standards (corrupting 1KB of data in each archive):
- **Gzip / Zip**: FAILED (Data loss, stream corruption).
- **XZ / 7-Zip**: FAILED (Data loss, CRC mismatch).
- **Lazarus**: SUCCESS (Detected damage via DNA fingerprints and automatically self-healed using the Phoenix Shield).

## Comparison vs Standard Tools

| Feature | Lazarus | XZ / 7-Zip | Gzip / Zip |
| :--- | :--- | :--- | :--- |
| **Compression Ratio** | Ultra High (LZMA L9) | Ultra High | Moderate |
| **Self-Healing** | **Yes (Phoenix Protocol)** | No | No |
| **Integrity Check** | Block-Level (Adaptive) | Stream-Level | File-Level |
| **Parallel Processing** | **Yes (via Rayon)** | Semi-Supported | Limited |
| **Repair Capability** | Mathematical (Wirehair) | External Rev-files only | None |
| **Speed** | Moderate (Parallel) | Slow (L9) | Very Fast |

### Pros
- **High Resilience**: Can survive partial file corruption (bit rot) and header damage that destroys standard archives.
- **Parallel Architecture**: Leverages multi-core CPUs for both compression and decompression.
- **Adaptive Precision**: Identifies granular data loss at the block level.
- **Native Directory Support**: Bundles folders without requiring external `tar` wrapping.

### Cons
- **Metadata Overhead**: Small archives (<2KB) are inefficient due to redundant header and parity requirements.
- **Context Loss**: Parallel chunking slightly reduces compression ratio vs monolithic streams.
- **Complexity**: Self-healing adds significant computational weight to the decoding process.

## Performance Benchmarks

*Recent tests conducted on x86_64 comparing Lazarus v0.1.6 vs Industry Standards.*

### Rigor Test (Compression & Integrity)
| File Type | Original | Lazarus | 7-Zip | Gzip | Healing (Chaos) |
| :--- | :--- | :--- | : :--- | :--- | :--- |
| **CSV Data** | 50 MB | 18.2 MB | 17.5 MB | 20.1 MB | **SUCCESS (Healed)** |
| **JSON Data**| 50 MB | 14.1 MB | 13.8 MB | 17.3 MB | **SUCCESS (Healed)** |
| **Mixed Payload**| 50 MB | 48.5 MB | 46.2 MB | 48.1 MB | **SUCCESS (Healed)** |
| **Server Logs** | 100 MB | 8.2 MB | 7.6 MB | 9.9 MB | **SUCCESS (Healed)** |

*\*Note: Healing is automatically disabled for archives < 2KB to prevent excessive metadata overhead.*


## Installation

### Debian/Ubuntu (.deb)
Download from the [Releases](https://github.com/1999AZZAR/lazarus/releases) page:
```bash
sudo dpkg -i lazarus_0.1.6_amd64.deb
```

### From Source
```bash
cargo build --release
sudo cp target/release/lazarus /usr/bin/
```

## Usage

Lazarus automatically detects whether you are providing a single file or an entire folder.

### Compress
```bash
lazarus compress <path>
```

### Decompress
```bash
lazarus decompress <file.lzr>
```

## License
[MIT License](LICENSE) - Copyright (c) 2026 Azzar Budiyanto

## Documentation
Comprehensive technical documentation, including roadmaps and performance analysis, can be found in the [docs/](docs/) directory.
