# Lazarus v0.1.5 Intense Benchmark Results
Tested against XZ (LZMA L9) and Gzip (DEFLATE L6) on x86_64.
Corruption Test: 1KB random data injected into archive at offset 5000.

| Tool | File Type | Ratio | Time (C) | Time (D) | Resilience (Chaos) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Lazarus** | server.log (50MB) | **99.63%** | 2.19s | 0.19s | **SUCCESS (Healed)** |
| XZ (L9) | server.log (50MB) | 99.99% | 0.75s | 0.07s | FAILED |
| Gzip (L6) | server.log (50MB) | 99.66% | 0.20s | 0.15s | FAILED |
| --- | --- | --- | --- | --- | --- |
| **Lazarus** | data.json (50MB) | **95.48%** | 3.68s | 0.21s | **SUCCESS (Healed)** |
| XZ (L9) | data.json (50MB) | 96.18% | 7.66s | 0.11s | FAILED |
| Gzip (L6) | data.json (50MB) | 94.79% | 0.31s | 0.15s | FAILED |
| --- | --- | --- | --- | --- | --- |
| **Lazarus** | random.bin (10MB) | **-5.93%** | 0.93s | 0.05s | **SUCCESS (Healed)** |
| XZ (L9) | random.bin (10MB) | -0.01% | 2.32s | 0.01s | FAILED |
| Gzip (L6) | random.bin (10MB) | -0.02% | 0.27s | 0.05s | FAILED |
| --- | --- | --- | --- | --- | --- |

## Observations
1. **Header Redundancy Triumph**: Unlike v0.1.4, the current version (v0.1.5) successfully survived 1KB corruption across all datasets. The "Brain Backup" protocol detected primary header corruption and resurrected structural metadata from the redundant copy.
2. **True Self-Healing**: Lazarus is the only tool in this test suite capable of bit-perfect data restoration after 1KB of direct stream corruption.
3. **Multi-Threaded Dominance**: On structured data (data.json), Lazarus v0.1.5 is ~2x faster than single-threaded XZ -9 while maintaining comparable compression density.
