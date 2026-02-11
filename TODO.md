# Lazarus Development Roadmap & Optimization Plan

## 🚀 Immediate Enhancements
- [ ] **Parallel Processing (Multithreading)**: Implement `rayon` for concurrent block compression/decompression to reduce bottleneck of LZMA Level 9.
- [ ] **Secret Shield (Encryption)**: Integrate AES-256-GCM or ChaCha20-Poly1305 to ensure data privacy alongside self-healing.
- [ ] **Enhanced Metadata**: Store file permissions (Unix) and original timestamps within the `.lzr` header.

## 🔬 Research & Experimental
- [ ] **WASM Support**: Port core engine to WebAssembly for browser-based secure file reconstruction.
- [ ] **Adaptive Parity**: Dynamically adjust Fountain Code overhead (5% to 20%) based on detected media type or user preference.
- [ ] **Deduplication Logic**: Add block-level deduplication for archiving identical data structures across large directories.

## 🛠️ Tooling & DevOps
- [ ] **Chaos Lab**: Create a Python/Rust stress-tester to inject random corruption at various scales and verify Phoenix Protocol reliability.
- [ ] **CLI Polish**: Add a progress bar (e.g., using `indicatif`) for better user feedback during heavy operations.
- [ ] **CI Pipeline**: Automate `.deb` and `.rpm` package builds on GitHub Actions.

---
*Initial roadmap drafted by Mema ❤️*
