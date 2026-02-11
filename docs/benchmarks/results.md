# Lazarus v0.1.6 Intense Benchmark Results
Tested against XZ (LZMA L9) and Gzip (DEFLATE L6) on x86_64.
Corruption Test: 1KB random data injected into archive at offset 5000.
Adaptive Parity: Dynamic scaling (3% - 10%) based on file size.

| Tool | File Type | Ratio | Time (C) | Time (D) | Resilience (Chaos) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Lazarus** | server.log (50MB) | **99.63%** | 2.32s | 0.21s | **SUCCESS (Healed)** |
| XZ (L9) | server.log (50MB) | 99.99% | 0.72s | 0.07s | FAILED |
| Gzip (L6) | server.log (50MB) | 99.66% | 0.19s | 0.15s | FAILED |
| --- | --- | --- | --- | --- | --- |
| **Lazarus** | data.json (50MB) | **95.48%** | 3.76s | 0.21s | **SUCCESS (Healed)** |
| XZ (L9) | data.json (50MB) | 96.18% | 7.62s | 0.11s | FAILED |
| Gzip (L6) | data.json (50MB) | 94.79% | 0.31s | 0.15s | FAILED |
| --- | --- | --- | --- | --- | --- |
| **Lazarus** | random.bin (10MB) | **-5.93%** | 0.88s | 0.05s | **SUCCESS (Healed)** |
| XZ (L9) | random.bin (10MB) | -0.01% | 2.31s | 0.01s | FAILED |
| Gzip (L6) | random.bin (10MB) | -0.02% | 0.27s | 0.05s | FAILED |
| --- | --- | --- | --- | --- | --- |

## Observations
1. **Adaptive Parity Stability**: The transition to dynamic parity (3-10%) did not compromise data integrity. Lazarus successfully healed all test archives despite 1KB of random corruption.
2. **Efficiency**: On medium-sized files (50MB), the system maintained a 5% overhead while providing bit-perfect restoration.
3. **Robustness**: The combination of "Brain Backup" (Header Redundancy) and Adaptive Parity ensures that Lazarus remains stable and self-healing across varying data scales.
