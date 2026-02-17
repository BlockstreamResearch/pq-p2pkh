use methods::ADDRESS_VERIFICATION_REGISTRY_GUEST_ID;
use risc0_zkvm::Receipt;

use std::time::Instant;
use std::{env, fs};

fn main() {
    let proof_path = env::args()
        .nth(1)
        .expect("Input proof path is not specified");

    let proof_file_content = fs::read_to_string(&proof_path).unwrap();

    let receipt: Receipt = serde_json::from_str(&proof_file_content).unwrap();

    let verification_start_time = Instant::now();

    receipt
        .verify(ADDRESS_VERIFICATION_REGISTRY_GUEST_ID)
        .unwrap();

    let verification_duration = verification_start_time.elapsed();
    println!("Verification completed in: {:?}", verification_duration);
}
