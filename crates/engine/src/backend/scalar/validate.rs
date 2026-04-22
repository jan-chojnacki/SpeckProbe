use crate::domain::key::Key;
use paste::paste;

macro_rules! define_validate {
    (
    bytes = $bytes:literal,
    vector = $vector:ty,
    comparator = $comparator:path,
    key_conversion = $key_conversion:ident,
    name = $name:tt
    ) => {
        paste! {
            #[inline(always)]
            pub fn [<ecb_validate_encrypt_ $name>]<const PREFIX: usize>(
                pt: &[[$vector; 2]],
                expected: &[[$vector; 2]],
                key: &Key<$bytes, PREFIX>,
            ) -> bool {
                for (p, e) in pt.iter().zip(expected) {
                    let result = speck::[<scalar_encrypt_block_ $name>](*p, key.$key_conversion());
                    if !(result[0] == e[0] && result[1] == e[1]) {
                        return false;
                    }
                }

                true
            }

            #[inline(always)]
            pub fn [<ecb_validate_decrypt_ $name>]<const PREFIX: usize>(
                ct: &[[$vector; 2]],
                expected: &[[$vector; 2]],
                key: &Key<$bytes, PREFIX>,
            ) -> bool {
                for (c, e) in ct.iter().zip(expected) {
                    let result = speck::[<scalar_decrypt_block_ $name>](*c, key.$key_conversion());
                    if !(result[0] == e[0] && result[1] == e[1]) {
                        return false;
                    }
                }

                true
            }
        }
    };
}

define_validate!(
    bytes = 8,
    vector = u16,
    comparator = block_compare,
    key_conversion = as_u16x4_le,
    name = 32_64
);
define_validate!(
    bytes = 9,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u24x3_le,
    name = 48_72
);
define_validate!(
    bytes = 12,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u24x4_le,
    name = 48_96
);
define_validate!(
    bytes = 12,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u32x3_le,
    name = 64_96
);
define_validate!(
    bytes = 16,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u32x4_le,
    name = 64_128
);
define_validate!(
    bytes = 12,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u48x2_le,
    name = 96_96
);
define_validate!(
    bytes = 18,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u48x3_le,
    name = 96_144
);
define_validate!(
    bytes = 16,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x2_le,
    name = 128_128
);
define_validate!(
    bytes = 24,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x3_le,
    name = 128_192
);
define_validate!(
    bytes = 32,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x4_le,
    name = 128_256
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::key::Key;
    use rstest::rstest;

    #[rstest]
    #[case([0x18, 0x19, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01], [[0x6574u16, 0x694c]], [[0xa868u16, 0x42f2]])]
    fn validate_32_64(
        #[case] key_bytes: [u8; 8],
        #[case] plain: [[u16; 2]; 1],
        #[case] cipher: [[u16; 2]; 1],
    ) {
        let key: Key<8, 0> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_32_64(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_32_64(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x10, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02], [[0x20796cu32, 0x6c6172]], [[0xc049a5u32, 0x385adc]])]
    fn validate_48_72(
        #[case] key_bytes: [u8; 9],
        #[case] plain: [[u32; 2]; 1],
        #[case] cipher: [[u32; 2]; 1],
    ) {
        let key: Key<9, 1> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_48_72(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_48_72(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x18, 0x19, 0x1a, 0x10, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02], [[0x6d2073u32, 0x696874]], [[0x735e10u32, 0xb6445d]])]
    fn validate_48_96(
        #[case] key_bytes: [u8; 12],
        #[case] plain: [[u32; 2]; 1],
        #[case] cipher: [[u32; 2]; 1],
    ) {
        let key: Key<12, 4> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_48_96(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_48_96(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03], [[0x74614620u32, 0x736e6165]], [[0x9f7952ecu32, 0x4175946c]])]
    fn validate_64_96(
        #[case] key_bytes: [u8; 12],
        #[case] plain: [[u32; 2]; 1],
        #[case] cipher: [[u32; 2]; 1],
    ) {
        let key: Key<12, 4> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_64_96(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_64_96(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x18, 0x19, 0x1a, 0x1b, 0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03], [[0x3b726574u32, 0x7475432d]], [[0x8c6fa548u32, 0x454e028b]])]
    fn validate_64_128(
        #[case] key_bytes: [u8; 16],
        #[case] plain: [[u32; 2]; 1],
        #[case] cipher: [[u32; 2]; 1],
    ) {
        let key: Key<16, 8> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_64_128(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_64_128(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05], [[0x65776f68202cu64, 0x656761737520]], [[0x9e4d09ab7178u64, 0x62bdde8f79aa]])]
    fn validate_96_96(
        #[case] key_bytes: [u8; 12],
        #[case] plain: [[u64; 2]; 1],
        #[case] cipher: [[u64; 2]; 1],
    ) {
        let key: Key<12, 4> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_96_96(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_96_96(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05], [[0x656d6974206eu64, 0x69202c726576]], [[0x2bf31072228au64, 0x7ae440252ee6]])]
    fn validate_96_144(
        #[case] key_bytes: [u8; 18],
        #[case] plain: [[u64; 2]; 1],
        #[case] cipher: [[u64; 2]; 1],
    ) {
        let key: Key<18, 10> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_96_144(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_96_144(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07], [[0x6c61766975716520u64, 0x7469206564616d20]], [[0xa65d985179783265u64, 0x7860fedf5c570d18]])]
    fn validate_128_128(
        #[case] key_bytes: [u8; 16],
        #[case] plain: [[u64; 2]; 1],
        #[case] cipher: [[u64; 2]; 1],
    ) {
        let key: Key<16, 8> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_128_128(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_128_128(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07], [[0x7261482066656968u64, 0x43206f7420746e65]], [[0x1be4cf3a13135566u64, 0xf9bc185de03c1886]])]
    fn validate_128_192(
        #[case] key_bytes: [u8; 24],
        #[case] plain: [[u64; 2]; 1],
        #[case] cipher: [[u64; 2]; 1],
    ) {
        let key: Key<24, 16> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_128_192(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_128_192(&cipher, &plain, &key));
    }

    #[rstest]
    #[case([0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07], [[0x65736f6874206e49u64, 0x202e72656e6f6f70]], [[0x4109010405c0f53eu64, 0x4eeeb48d9c188f43]])]
    fn validate_128_256(
        #[case] key_bytes: [u8; 32],
        #[case] plain: [[u64; 2]; 1],
        #[case] cipher: [[u64; 2]; 1],
    ) {
        let key: Key<32, 24> = Key::new_from_bytes(&key_bytes);
        assert!(ecb_validate_encrypt_128_256(&plain, &cipher, &key));
        assert!(ecb_validate_decrypt_128_256(&cipher, &plain, &key));
    }
}
