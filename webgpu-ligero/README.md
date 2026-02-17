# WebGPU Ligero Bitcoin Address Ownership Verification Benchmark

This project benchmarks zero-knowledge proof generation and verification for Bitcoin address ownership using Ligero.

## Prerequisites

- Rust toolchain
- [Cairo](https://www.cairo-lang.org/tutorial/getting-started-with-cairo/#installing-cairo)
- [Ligero](https://github.com/ligeroinc/ligero-prover)

## Running the Benchmarks

### Build the program

To build the STWO-CAIRO program, run:

```bash
cargo build --release --target wasm32-unknown-unknown
```

### Proof Generation

To generate a zero-knowledge proof and measure the time taken:

```bash
export SHARERS_PATH=""

time webgpu_prover '{"program":"target/wasm32-unknown-unknown/release/ligero.wasm","shader-path":'"$SHARERS_PATH"',"packing":8192,"private-indices":[],"args":[]}'

```

### Proof Verification

To verify a previously generated proof and measure the verification time:

```bash
export SHARERS_PATH=""

time webgpu_verifier '{"program":"target/wasm32-unknown-unknown/release/ligero.wasm","shader-path":'"$SHARERS_PATH"',"packing":8192,"private-indices":[],"args":[]}'
```
