# STWO-CAIRO Bitcoin Address Ownership Verification Benchmark

This project benchmarks zero-knowledge proof generation and verification for Bitcoin address ownership using STWO-CAIRO.

## Prerequisites

- **Scarb 2.14.0** — the Cairo package manager (includes Cairo 2.14.0)
- **Rust nightly** — required for building the prover
- **Git**

### Install Scarb 2.14.0

```bash
curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh -s -- -v 2.14.0
```

Or via [asdf](https://asdf-vm.com/):

```bash
asdf install scarb 2.14.0
asdf set scarb 2.14.0
```

Verify:

```bash
scarb --version
# scarb 2.14.0 ... cairo: 2.14.0
```

### Build cairo-prove

The prover binary needs to be built from the [stwo-cairo](https://github.com/starkware-libs/stwo-cairo) repository:

```bash
git clone https://github.com/starkware-libs/stwo-cairo.git
cd stwo-cairo
git checkout d2108196

cd cairo-prove
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release
```

Add to your PATH:

```bash
export PATH="$(pwd)/target/release:$PATH"
```

Verify:

```bash
cairo-prove --help
```

## Running the Benchmarks

### Build the program

```bash
scarb build
```

### Execute

```bash
scarb execute
```

### Proof Generation

```bash
time cairo-prove prove target/dev/cairo_tee_replacement.executable.json ./proof.json
```

### Proof Verification

```bash
time cairo-prove verify proof.json
```

## Version Pinning

This project depends on [garaga](https://github.com/keep-starknet-strange/garaga) for elliptic curve operations. The `Scarb.lock` pins garaga to a specific commit compatible with Cairo 2.14. **Do not** upgrade Scarb or garaga without verifying compatibility — newer Cairo versions (2.15+) introduce corelib changes that break garaga.