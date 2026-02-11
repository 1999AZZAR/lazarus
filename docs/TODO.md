# Lazarus Development Roadmap & Optimization Plan

## 1. Project Analysis Summary

### Pros
- **Extreme Resilience**: Native self-healing via Fountain Codes (Wirehair) with bit-perfect reconstruction.
- **Parallel Performance**: Multi-threaded compression and decompression via `rayon` integration.
- **Granular Integrity**: Block-level CRC-32 fingerprints for precise corruption detection.
- **Adaptive Scaling**: Dynamic block sizing for balanced metadata overhead.

### Cons
- **Context Loss**: Independent chunk compression slightly reduces the overall compression ratio compared to monolithic streams.
- **Static Parity**: Fixed 5% overhead may not fit all storage scenarios (too high for reliable media, too low for failing hardware).
- **Security Gap**: Data is stored in cleartext (after compression), making it vulnerable to unauthorized access.

## 2. Completed Milestones
- [x] **Parallel Processing**: Successfully implemented `rayon` for concurrent chunk handling.
- [x] **Chaos Lab**: Built a comprehensive stress-testing suite (`chaos_test.sh` and `chaos_test.rs`).
- [x] **Performance Tracking**: Documented impact analysis and benchmarking results.
- [x] **Header Redundancy**: Implemented "Brain Backup" (Redundant Headers) and DNA verification to prevent segfaults during corruption.
- [x] **Adaptive Parity**: Implemented dynamic recovery overhead (3% - 10%) optimized for varying file sizes.

## 3. Active Technical Debt

### Problem: Security Vulnerability (Cleartext Data)
- **Solution**: Integrate a "Secret Shield" using ChaCha20-Poly1305 encryption. Encrypt blocks before applying Fountain Code parity.

### Problem: Missing System Metadata
- **Solution**: Expand the `.lzr` header to store and restore Unix file permissions (mode) and system timestamps (mtime).

## 4. Future Roadmap

### Phase 1: Security, Portability & Robustness
- [ ] Implement ChaCha20-Poly1305 encryption layer.
- [ ] Add Unix metadata persistence (permissions, timestamps).
- [ ] **WASM Porting**: Compile core engine for browser-side data resurrection.

### Phase 2: Intelligence & Optimization
- [ ] **Deduplication**: Block-level deduplication for redundant data archiving.
- [ ] **CPU Tuning**: Auto-detect core count for optimal chunk size allocation.

### Phase 3: UX & Infrastructure
- [ ] **CLI Polish**: Add real-time progress bars using `indicatif`.
- [ ] **CI/CD Pipeline**: Automate multi-platform binary and package builds.

---
*Updated by Mema after Header Redundancy implementation*
