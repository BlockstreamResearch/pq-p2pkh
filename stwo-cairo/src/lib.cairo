mod sha256lib;

use core::byte_array::ByteArray;

use garaga::signatures::ecdsa::{is_valid_ecdsa_signature, ECDSASignatureWithHint};

use sha256lib::sha256;

#[executable]
fn main() -> ByteArray {
    let mut ecdsa_sig_with_hints_serialized: Span<felt252> = array![7021492998533709890132676858, 22430033418337142135295413153, 16945164401844010816, 0, 24255496522932945192754094102121042923, 166555602851724256161958524654797996893, 1, 59580082805284715453571721528, 16114937605692213869522435538, 13752098491662096469, 0, 7793379924885316643324926572, 34674194179057787003962635973, 15987569902268145005, 0, 314595223475647964805660839007167014148, 173530984672212808755928504668995328537, 24, 32552139938002044694355188123, 35734336635075342549234703887, 1427647248988240872, 0, 11477132974211901003657557349, 73970888002084857312111693445, 16472136545597985743, 0, 7521512482481483739, 340282366920938463465629239333481265712, 137077072629558622, 340282366920938463473077489060604378281, 15224498051916937071570259363, 5540953489653152574334369019, 2335575233255137155, 0, 14202738530825166975458160277, 15121214651804025935429673251, 13990380248628913288, 0, 7141630864550785563, 340282366920938463470404935075725069385, 16906764447043917203, 9894887033457728558].span();

    let ecdsa_with_hints = Serde::<
        ECDSASignatureWithHint,
    >::deserialize(ref ecdsa_sig_with_hints_serialized)
        .unwrap();

    
    let pk_x: u256 = 0xbed944b767b458553411f6c613f752476a6fbdd2c0837d48b7c187eb38e8e938;
    let pk_y: u256 = 0xdddf421ece6b5d6d7009d238e7eb4e42c5ffb6c5192e8864473d48eebbef9a6c;

    address_registry_check(        
        pk_x,
        pk_y,
        ecdsa_with_hints
    )
}

fn address_registry_check(
    pk_x: u256,
    pk_y: u256,
    sig: ECDSASignatureWithHint,
) -> ByteArray {
    assert!(is_valid_ecdsa_signature(sig, 2), "Signature verification failed");

    let part_address = compute_part_address(pk_x, pk_y);

    part_address
}

fn compute_part_address(
    pk_x: u256,
    pk_y: u256,
) -> ByteArray {
    let mut pk: ByteArray = "";
    pk.append(@pk_x.into());
    pk.append(@pk_y.into());

    let sha256_hash = sha256_byte_array(@pk);

    sha256_hash
}

pub fn sha256_byte_array(byte: @ByteArray) -> ByteArray {
    let array_u8 = byte_array_to_array_u8(byte);

    let msg_hash = sha256(array_u8);
    let mut hash_value: ByteArray = "";
    for word in msg_hash {
        hash_value.append_word((*word).into(), 4);
    };

    hash_value
}

pub fn byte_array_to_array_u8(byte: @ByteArray) -> Array<u8> {
    let mut result = array![];
    for i in 0..byte.len() {
        let byte_value = byte.at(i).unwrap();
        result.append(byte_value.into());
    }
    
    result
}

pub impl U256IntoByteArray of Into<u256, ByteArray> {
    fn into(self: u256) -> ByteArray {
        let mut ba = Default::default();
        ba.append_word(self.high.into(), 16);
        ba.append_word(self.low.into(), 16);
        ba
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ecdsa_sig_with_hints_serialized: Span<felt252> = array![7021492998533709890132676858, 22430033418337142135295413153, 16945164401844010816, 0, 24255496522932945192754094102121042923, 166555602851724256161958524654797996893, 1, 59580082805284715453571721528, 16114937605692213869522435538, 13752098491662096469, 0, 7793379924885316643324926572, 34674194179057787003962635973, 15987569902268145005, 0, 314595223475647964805660839007167014148, 173530984672212808755928504668995328537, 24, 32552139938002044694355188123, 35734336635075342549234703887, 1427647248988240872, 0, 11477132974211901003657557349, 73970888002084857312111693445, 16472136545597985743, 0, 7521512482481483739, 340282366920938463465629239333481265712, 137077072629558622, 340282366920938463473077489060604378281, 15224498051916937071570259363, 5540953489653152574334369019, 2335575233255137155, 0, 14202738530825166975458160277, 15121214651804025935429673251, 13990380248628913288, 0, 7141630864550785563, 340282366920938463470404935075725069385, 16906764447043917203, 9894887033457728558].span();

        let ecdsa_with_hints = Serde::<ECDSASignatureWithHint>::deserialize(ref ecdsa_sig_with_hints_serialized).unwrap();

        let pk_x: u256 = 0xbed944b767b458553411f6c613f752476a6fbdd2c0837d48b7c187eb38e8e938;
        let pk_y: u256 = 0xdddf421ece6b5d6d7009d238e7eb4e42c5ffb6c5192e8864473d48eebbef9a6c;

        let part_address = address_registry_check(
            pk_x,
            pk_y,
            ecdsa_with_hints
        );

        println!("Part Address: {}", part_address);
    }
}
