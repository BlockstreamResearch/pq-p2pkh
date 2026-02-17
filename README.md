# Post-quantum provers for P2PKH output
Benchmarks of post-quantum provers for P2PKH outputs

| Proving System | Proving Time | Verification Time | Memory usage| Proof Size | Notes |
|---|---|---|---|---|---|
| STWO-Cairo | 8 s | 50 ms | 6 GB | 5.6 MB | Cairo program, STWO prover |
| Ligero (Apple M2 Max) | 4 s | 2 s | 1.8 GB | 4.2 MB | Runs on Mac|
| Ligero (WebGPU) | 22 s | 4 s | 1.9 GB | 4.2 MB | Runs in Chromium browser |
| RISC Zero | 2 m | 100 ms | 1.2 GB | 2 MB | RISC-V zkVM |
| SP1 | 1 m | 2 s | 16 GB | 10 MB | Succinct's zkVM |
| Valida (Lita) | 4.5 m | 2 s | 12 GB | 6 MB | Plonky3 backend |