use std::env;
use std::fs;
use std::time::Instant;

use methods::ADDRESS_VERIFICATION_REGISTRY_GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};

static PUBLIC_KEY: [u8; 64] = [
    190, 217, 68, 183, 103, 180, 88, 85, 52, 17, 246, 198, 19, 247, 82, 71, 106, 111, 189, 210,
    192, 131, 125, 72, 183, 193, 135, 235, 56, 232, 233, 56, 221, 223, 66, 30, 206, 107, 93, 109,
    112, 9, 210, 56, 231, 235, 78, 66, 197, 255, 182, 197, 25, 46, 136, 100, 71, 61, 72, 238, 187,
    239, 154, 108,
];

static SIGNATURE: [u8; 64] = [
    235, 41, 81, 62, 128, 129, 191, 64, 72, 121, 176, 84, 13, 222, 185, 53, 191, 1, 139, 161, 22,
    176, 10, 250, 39, 180, 168, 32, 54, 208, 144, 250, 125, 77, 113, 58, 41, 244, 18, 192, 117,
    116, 224, 94, 71, 148, 67, 93, 18, 63, 112, 79, 25, 65, 129, 168, 246, 191, 174, 96, 183, 196,
    83, 235,
];

static MESSAGE_HASH: [u8; 32] = [
    130, 140, 218, 26, 112, 75, 12, 41, 197, 151, 204, 5, 96, 174, 26, 25, 236, 172, 214, 12, 78,
    171, 29, 151, 59, 197, 116, 27, 50, 221, 101, 4,
];

fn main() {
    let proof_path = env::args()
        .nth(1)
        .expect("Output proof path is not specified");

    let env = ExecutorEnv::builder()
        .write(&PUBLIC_KEY.to_vec())
        .unwrap()
        .write(&SIGNATURE.to_vec())
        .unwrap()
        .write(&MESSAGE_HASH.to_vec())
        .unwrap()
        .build()
        .unwrap();

    let proof_start_time = Instant::now();

    let opts = ProverOpts::default();

    let prove_info = default_prover()
        .prove_with_opts(env, ADDRESS_VERIFICATION_REGISTRY_GUEST_ELF, &opts)
        .unwrap();

    let proof_duration = proof_start_time.elapsed();
    println!("Proof generated in: {:?}", proof_duration);

    let receipt = prove_info.receipt;

    let receipt_json = serde_json::to_string(&receipt).unwrap();

    fs::write(proof_path, &receipt_json).unwrap();
}
