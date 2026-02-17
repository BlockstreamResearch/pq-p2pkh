# STWO-CAIRO Bitcoin Address Ownership Verification Benchmark

This project benchmarks zero-knowledge proof generation and verification for Bitcoin address ownership using STWO-CAIRO.

## Prerequisites

- [Cairo](https://www.cairo-lang.org/tutorial/getting-started-with-cairo/#installing-cairo)
- [STWO-CAIRO](https://github.com/starkware-libs/stwo-cairo)

## Running the Benchmarks

### Build the program

To build the STWO-CAIRO program, run:

```bash
scarb build
```

### Proof Generation

To generate a zero-knowledge proof and measure the time taken:

```bash
time cairo-prove prove target/dev/cairo_tee_replacement.executable.json ./proof.json
```

### Proof Verification

To verify a previously generated proof and measure the verification time:

```bash
time cairo-prove verify proof.json
```
