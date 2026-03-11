#!/bin/bash
PROVER=~/mjthatch/proving-utils/target/release/stwo-run-and-prove
PROGRAM=$(pwd)/target/dev/cairo_tee_replacement.executable.json

echo "=== Proving ==="
time $PROVER \
  --program "$PROGRAM" \
  --proof_path ./proof.json

echo "=== Verifying ==="
time $PROVER \
  --program "$PROGRAM" \
  --proof_path ./proof.json \
  --verify
