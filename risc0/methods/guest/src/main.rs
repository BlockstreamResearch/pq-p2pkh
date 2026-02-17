use k256::ecdsa::signature::hazmat::PrehashVerifier;
use k256::ecdsa::{Signature, VerifyingKey};

use k256::sha2::{Digest, Sha256};

use risc0_zkvm::guest::env;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct GuestOutputs {
    pubkey_hash: Vec<u8>,
    msg_hash: Vec<u8>,
}

fn main() {
    let pk_bytes: Vec<u8> = env::read();
    let sig_bytes: Vec<u8> = env::read();
    let msg_hash_bytes: Vec<u8> = env::read();

    if !verify_signature(&msg_hash_bytes, &sig_bytes, &pk_bytes) {
        panic!("Signature verification failed");
    }

    let outputs = GuestOutputs {
        pubkey_hash: Sha256::digest(&pk_bytes).to_vec(),
        msg_hash: msg_hash_bytes,
    };

    env::commit(&outputs);
}

pub fn verify_signature(msg_hash: &[u8], sig_bytes: &[u8], pk_bytes: &[u8]) -> bool {
    let mut uncompressed_pk = vec![0x04];
    uncompressed_pk.extend_from_slice(pk_bytes);

    let encoded_point_pk = k256::EncodedPoint::from_bytes(&uncompressed_pk).unwrap();

    let verifying_key = VerifyingKey::from_encoded_point(&encoded_point_pk).unwrap();

    let signature = Signature::from_bytes(sig_bytes.into()).unwrap();

    verifying_key.verify_prehash(msg_hash, &signature).is_ok()
}
