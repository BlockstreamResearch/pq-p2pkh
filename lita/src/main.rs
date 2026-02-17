#![no_main]

use k256::ecdsa::signature::hazmat::PrehashVerifier;
use k256::ecdsa::{Signature, VerifyingKey};

use k256::sha2::{Digest, Sha256};

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

#[no_mangle]
pub fn main() {
    if !verify_signature(&MESSAGE_HASH, &SIGNATURE, &PUBLIC_KEY) {
        panic!("Signature verification failed");
    }

    let _ = Sha256::digest(&PUBLIC_KEY);
}

pub fn verify_signature(msg_hash: &[u8], sig_bytes: &[u8], pk_bytes: &[u8]) -> bool {
    let mut uncompressed_pk = vec![0x04];
    uncompressed_pk.extend_from_slice(pk_bytes);

    let encoded_point_pk = k256::EncodedPoint::from_bytes(&uncompressed_pk).unwrap();

    let verifying_key = VerifyingKey::from_encoded_point(&encoded_point_pk).unwrap();

    let signature = Signature::from_bytes(sig_bytes.into()).unwrap();

    verifying_key.verify_prehash(msg_hash, &signature).is_ok()
}
