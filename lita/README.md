# Lita Bitcoin Address Ownership Verification Benchmark

This project benchmarks zero-knowledge proof generation and verification for Bitcoin address ownership using Lita.

## Prerequisites

- Rust toolchain
- [Lita toolchain](https://lita.gitbook.io/lita-documentation/quick-start/tutorial-rust-via-docker)

## Running the Benchmarks

### Build the program

To build the STWO-CAIRO program, run:

```bash
cargo +valida build --release
```

### Proof Generation

To generate a zero-knowledge proof and measure the time taken:

```bash
time valida prove ./target/valida-unknown-baremetal-gnu/release/btc_addr_ownership_check proof empty_stdio.txt
```

### Proof Verification

To verify a previously generated proof and measure the verification time:

```bash
time valida verify ./target/valida-unknown-baremetal-gnu/release/btc_addr_ownership_check proof empty_stdio.txt
```
