# RISC Zero Bitcoin Address Ownership Verification Benchmark

This project benchmarks zero-knowledge proof generation and verification for Bitcoin address ownership using RISC Zero.

## Prerequisites

- Rust toolchain
- [RISC Zero toolchain](https://dev.risczero.com/api/zkvm/install)

## Running the Benchmarks

### Proof Generation

To generate a zero-knowledge proof and measure the time taken:

```bash
RISC0_DEV_MODE=0 cargo run --bin prover --release -- proof.json
```

### Proof Verification

To verify a previously generated proof and measure the verification time:

```bash
RISC0_DEV_MODE=0 cargo run --bin verifier --release -- proof.json
```
