use std::env;
use std::time::Instant;

use sp1_sdk::{include_elf, Prover, ProverClient, SP1ProofWithPublicValues};

pub const P2PKH_ELF: &[u8] = include_elf!("p2pkh-program");

fn main() {
    let proof_path = env::args()
        .nth(1)
        .expect("Output proof path is not specified");

    let proof = SP1ProofWithPublicValues::load(&proof_path).unwrap();

    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(P2PKH_ELF);

    let verification_start_time = Instant::now();

    prover.verify(&proof, &vk).unwrap();

    let verification_duration = verification_start_time.elapsed();
    println!("Verification completed in: {:?}", verification_duration);
}
