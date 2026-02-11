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

## 3. Active Technical Debt

### Problem: Header Vulnerability to Corruption
- **Issue**: Intense chaos testing revealed that random corruption (1KB) on larger archives (50MB) can lead to segmentation faults or total decompression failure if it hits the archive header or block metadata. Currently, the Phoenix Shield only protects the data payload, leaving the structural "brain" of the archive vulnerable.
- **Solution**: Implement "Header Parity" or a redundant metadata strategy. Key structural information (chunk sizes, fingerprints, and Wirehair parameters) should be stored with its own error-correction overhead or mirrored at the end of the archive.

### Problem: Resource Usage on Heavy Datasets
- **Solution**: Integrate a "Secret Shield" using ChaCha20-Poly1305 encryption. Encrypt blocks before applying Fountain Code parity.

### Problem: Missing System Metadata
- **Solution**: Expand the `.lzr` header to store and restore Unix file permissions (mode) and system timestamps (mtime).

## 4. Future Roadmap

### Phase 1: Security, Portability & Robustness
- [ ] Implement ChaCha20-Poly1305 encryption layer.
- [ ] Add Unix metadata persistence (permissions, timestamps).
- [ ] **Header Redundancy**: Implement parity-protected headers or redundant metadata blocks to prevent "single point of failure" corruption.
- [ ] **WASM Porting**: Compile core engine for browser-side data resurrection.

### Phase 2: Intelligence & Optimization
- [ ] **Adaptive Parity**: Dynamic adjustment of recovery overhead (5% to 25%).
- [ ] **Deduplication**: Block-level deduplication for redundant data archiving.
- [ ] **CPU Tuning**: Auto-detect core count for optimal chunk size allocation.

### Phase 3: UX & Infrastructure
- [ ] **CLI Polish**: Add real-time progress bars using `indicatif`.
- [ ] **CI/CD Pipeline**: Automate multi-platform binary and package builds.

---
*Updated by Mema after Rayon & Chaos Lab implementation*
