# Lazarus Development Roadmap & Optimization Plan

## 1. Project Analysis Summary

### Pros
- **High Resilience**: Native self-healing via Fountain Codes (Wirehair) allows bit-perfect reconstruction from corrupted states.
- **Precision Integrity**: Block-level CRC-32 fingerprints provide granular detection of where corruption occurred.
- **Extreme Density**: Utilization of LZMA Level 9 achieves industry-leading compression ratios for cold storage.
- **Adaptive Architecture**: Scales block sizes dynamically to balance metadata overhead.

### Cons
- **Performance Bottleneck**: Single-threaded LZMA Level 9 is computationally expensive and slow.
- **Static Overhead**: Fixed 5% parity might be excessive for small files or insufficient for highly unstable storage media.
- **Resource Heavy**: High memory and CPU usage during the compression/decompression phase.

## 2. Immediate Technical Debt & Fixes

### Problem: Performance Bottleneck in Compression
- **Issue**: Current implementation processes blocks sequentially, wasting multi-core CPU potential.
- **Solution**: Implement `rayon` for parallel block processing. Each chunk can be compressed independently, significantly reducing total wall-clock time.

### Problem: Security Vulnerability (Cleartext Data)
- **Issue**: While the data is resilient to corruption, it is not protected from unauthorized access.
- **Solution**: Integrate a "Secret Shield" using ChaCha20-Poly1305 encryption. Encrypt blocks before applying Fountain Code parity to ensure both privacy and resilience.

### Problem: Missing Unix Metadata
- **Issue**: File permissions and original timestamps are lost during the archive process.
- **Solution**: Expand the `.lzr` header structure to store and restore Unix file permissions (mode) and system timestamps (mtime).

## 3. Development Roadmap

### Phase 1: Performance & Security
- [ ] Implement parallel processing via `rayon`.
- [ ] Add AES-256-GCM or ChaCha20-Poly1305 encryption layer.
- [ ] Standardize header format for Unix metadata persistence.

### Phase 2: Experimental & Research
- [ ] **WASM Porting**: Compile core engine to WebAssembly for browser-side data resurrection.
- [ ] **Adaptive Parity**: Implement logic to dynamically adjust recovery overhead (5% to 25%) based on data importance or user input.
- [ ] **Deduplication**: Add block-level deduplication to optimize archives containing redundant data structures.

### Phase 3: Infrastructure & Tooling
- [ ] **Chaos Lab**: Develop a dedicated stress-testing suite to automate random corruption injection and verification.
- [ ] **CLI Polish**: Integrate `indicatif` for real-time progress bars and throughput metrics.
- [ ] **CI Pipeline**: Automate multi-platform builds (.deb, .rpm, binary) using GitHub Actions.

---
*Roadmap drafted by Mema*
