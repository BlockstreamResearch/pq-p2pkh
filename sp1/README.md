# SP1 Bitcoin Address Ownership Verification Benchmark

This project benchmarks zero-knowledge proof generation and verification for Bitcoin address ownership using SP1.

## Prerequisites

- Rust toolchain
- [SP1 toolchain](https://docs.succinct.xyz/docs/sp1/getting-started/install)

## Running the Benchmarks

### Proof Generation

To generate a zero-knowledge proof and measure the time taken:

```bash
cargo run --bin prover --release -- proof
```

### Proof Verification

To verify a previously generated proof and measure the verification time:

```bash
cargo run --bin verifier --release -- proof
```
