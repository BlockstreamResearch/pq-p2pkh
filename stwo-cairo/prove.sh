#!/bin/bash
if ! command -v cairo-prove &> /dev/null; then
    echo "Error: cairo-prove not found in PATH"
    echo ""
    echo "Build it from stwo-cairo:"
    echo "  git clone https://github.com/starkware-libs/stwo-cairo.git"
    echo "  cd stwo-cairo && git checkout d2108196"
    echo "  cd cairo-prove"
    echo '  RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release'
    echo '  export PATH="$(pwd)/target/release:$PATH"'
    exit 1
fi

echo "=== Proving ==="
time cairo-prove prove target/dev/cairo_tee_replacement.executable.json ./proof.json

echo "=== Verifying ==="
time cairo-prove verify proof.json