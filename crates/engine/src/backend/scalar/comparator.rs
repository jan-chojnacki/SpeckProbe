use crate::domain::key::Key;

#[inline(always)]
pub fn scalar_block_compare<T, const BYTES: usize, const PREFIX: usize>(
    e: &[T; 2],
    v: &[T; 2],
    key: &Key<BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) where
    T: PartialEq,
{
    if v[0] == e[0] && v[1] == e[1] {
        out.push(*key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const KEY_BYTES_U16: &[u8; 8] = &[0x18, 0x19, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01];

    #[rstest]
    #[case([0xa868u16, 0x42f2])]
    fn scalar_u16_hit(#[case] expected_data: [u16; 2]) {
        let key: Key<8, 0> = Key::new_from_bytes(KEY_BYTES_U16);
        let mut out = Vec::new();
        scalar_block_compare(&expected_data, &expected_data, &key, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].as_bytes(), key.as_bytes());
    }

    #[rstest]
    #[case([0xa868u16, 0x42f2], [0x0000u16, 0x0000])]
    #[case([0xa868u16, 0x42f2], [0xa868u16, 0x0000])]
    fn scalar_u16_miss(#[case] expected_data: [u16; 2], #[case] value_data: [u16; 2]) {
        let key: Key<8, 0> = Key::new_from_bytes(KEY_BYTES_U16);
        let mut out = Vec::new();
        scalar_block_compare(&expected_data, &value_data, &key, &mut out);
        assert!(out.is_empty());
    }

    const KEY_BYTES_U32: &[u8; 12] = &[
        0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03,
    ];

    #[rstest]
    #[case([0x9f7952ecu32, 0x4175946c])]
    fn scalar_u32_hit(#[case] expected_data: [u32; 2]) {
        let key: Key<12, 4> = Key::new_from_bytes(KEY_BYTES_U32);
        let mut out = Vec::new();
        scalar_block_compare(&expected_data, &expected_data, &key, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].as_bytes(), key.as_bytes());
    }

    #[rstest]
    #[case([0x9f7952ecu32, 0x4175946c], [0x00000000u32, 0x00000000])]
    #[case([0x9f7952ecu32, 0x4175946c], [0x9f7952ecu32, 0x00000000])]
    fn scalar_u32_miss(#[case] expected_data: [u32; 2], #[case] value_data: [u32; 2]) {
        let key: Key<12, 4> = Key::new_from_bytes(KEY_BYTES_U32);
        let mut out = Vec::new();
        scalar_block_compare(&expected_data, &value_data, &key, &mut out);
        assert!(out.is_empty());
    }

    const KEY_BYTES_U64: &[u8; 16] = &[
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
        0x07,
    ];

    #[rstest]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18])]
    fn scalar_u64_hit(#[case] expected_data: [u64; 2]) {
        let key: Key<16, 8> = Key::new_from_bytes(KEY_BYTES_U64);
        let mut out = Vec::new();
        scalar_block_compare(&expected_data, &expected_data, &key, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].as_bytes(), key.as_bytes());
    }

    #[rstest]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18], [0x0000000000000000u64, 0x0000000000000000])]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18], [0xa65d985179783265u64, 0x0000000000000000])]
    fn scalar_u64_miss(#[case] expected_data: [u64; 2], #[case] value_data: [u64; 2]) {
        let key: Key<16, 8> = Key::new_from_bytes(KEY_BYTES_U64);
        let mut out = Vec::new();
        scalar_block_compare(&expected_data, &value_data, &key, &mut out);
        assert!(out.is_empty());
    }
}
