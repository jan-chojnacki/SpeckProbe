use super::AVX512Key;
use crate::search::domain::key::Key;
use std::arch::x86_64::{
    __m512i, _mm512_cmpeq_epi16_mask, _mm512_cmpeq_epi32_mask, _mm512_cmpeq_epi64_mask,
};

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512bw")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
pub fn avx512_block_compare_u16<const BYTES: usize, const PREFIX: usize>(
    e: &[__m512i; 2],
    v: &[__m512i; 2],
    key: &AVX512Key<32, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm512_cmpeq_epi16_mask(e[0], v[0]);
    let cmp_hi = _mm512_cmpeq_epi16_mask(e[1], v[1]);
    let mut lanes = cmp_lo & cmp_hi;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx512f` before calling this function."]
pub fn avx512_block_compare_u32<const BYTES: usize, const PREFIX: usize>(
    e: &[__m512i; 2],
    v: &[__m512i; 2],
    key: &AVX512Key<16, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm512_cmpeq_epi32_mask(e[0], v[0]);
    let cmp_hi = _mm512_cmpeq_epi32_mask(e[1], v[1]);
    let mut lanes = cmp_lo & cmp_hi;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx512f` before calling this function."]
pub fn avx512_block_compare_u64<const BYTES: usize, const PREFIX: usize>(
    e: &[__m512i; 2],
    v: &[__m512i; 2],
    key: &AVX512Key<8, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm512_cmpeq_epi64_mask(e[0], v[0]);
    let cmp_hi = _mm512_cmpeq_epi64_mask(e[1], v[1]);
    let mut lanes = cmp_lo & cmp_hi;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    use super::*;
    use crate::search::executor::backend::x86_64::avx512::converter::{
        avx512_u16x2_block_to_vec, avx512_u32x2_block_to_vec, avx512_u64x2_block_to_vec,
    };
    use crate::speck::SpeckVersion;
    use rstest::rstest;

    const KEY_U16: &[u8; 8] = &[0x18, 0x19, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01];
    const V_U16: u64 = 0x0100_0908_1110_1918;

    #[rstest]
    #[case([0xa868u16, 0x42f2])]
    fn avx512_u16_hit(#[case] expected_data: [u16; 2]) {
        if !is_x86_feature_detected!("avx512bw") {
            return;
        }
        unsafe {
            let expected = avx512_u16x2_block_to_vec(expected_data);
            let key = AVX512Key::<32, 8, 0>::new(&[], [V_U16; 32], SpeckVersion::Speck32_64);
            let mut out = Vec::new();
            avx512_block_compare_u16(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 32);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U16.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0xa868u16, 0x42f2], [0x0000u16, 0x0000])]
    #[case([0xa868u16, 0x42f2], [0xa868u16, 0x0000])]
    fn avx512_u16_miss(#[case] expected_data: [u16; 2], #[case] value_data: [u16; 2]) {
        if !is_x86_feature_detected!("avx512bw") {
            return;
        }
        unsafe {
            let expected = avx512_u16x2_block_to_vec(expected_data);
            let v = avx512_u16x2_block_to_vec(value_data);
            let key = AVX512Key::<32, 8, 0>::new(&[], [V_U16; 32], SpeckVersion::Speck32_64);
            let mut out = Vec::new();
            avx512_block_compare_u16(&expected, &v, &key, &mut out);
            assert!(out.is_empty());
        }
    }

    const KEY_U32: &[u8; 12] = &[
        0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03,
    ];
    const V_U32: u64 = 0x0b0a_0908_1312_1110;
    const PREFIX_U32: &[u8; 4] = &[0x00, 0x01, 0x02, 0x03];

    #[rstest]
    #[case([0x9f7952ecu32, 0x4175946c])]
    fn avx512_u32_hit(#[case] expected_data: [u32; 2]) {
        if !is_x86_feature_detected!("avx512f") {
            return;
        }
        unsafe {
            let expected = avx512_u32x2_block_to_vec(expected_data);
            let key =
                AVX512Key::<16, 12, 4>::new(PREFIX_U32, [V_U32; 16], SpeckVersion::Speck64_96);
            let mut out = Vec::new();
            avx512_block_compare_u32(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 16);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U32.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0x9f7952ecu32, 0x4175946c], [0x00000000u32, 0x00000000])]
    #[case([0x9f7952ecu32, 0x4175946c], [0x9f7952ecu32, 0x00000000])]
    fn avx512_u32_miss(#[case] expected_data: [u32; 2], #[case] value_data: [u32; 2]) {
        if !is_x86_feature_detected!("avx512f") {
            return;
        }
        unsafe {
            let expected = avx512_u32x2_block_to_vec(expected_data);
            let v = avx512_u32x2_block_to_vec(value_data);
            let key =
                AVX512Key::<16, 12, 4>::new(PREFIX_U32, [V_U32; 16], SpeckVersion::Speck64_96);
            let mut out = Vec::new();
            avx512_block_compare_u32(&expected, &v, &key, &mut out);
            assert!(out.is_empty());
        }
    }

    const KEY_U64: &[u8; 16] = &[
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
        0x07,
    ];
    const V_U64: u64 = 0x0f0e_0d0c_0b0a_0908;
    const PREFIX_U64: &[u8; 8] = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];

    #[rstest]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18])]
    fn avx512_u64_hit(#[case] expected_data: [u64; 2]) {
        if !is_x86_feature_detected!("avx512f") {
            return;
        }
        unsafe {
            let expected = avx512_u64x2_block_to_vec(expected_data);
            let key =
                AVX512Key::<8, 16, 8>::new(PREFIX_U64, [V_U64; 8], SpeckVersion::Speck128_128);
            let mut out = Vec::new();
            avx512_block_compare_u64(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 8);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U64.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18], [0x0u64, 0x0])]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18], [0xa65d985179783265u64, 0x0])]
    fn avx512_u64_miss(#[case] expected_data: [u64; 2], #[case] value_data: [u64; 2]) {
        if !is_x86_feature_detected!("avx512f") {
            return;
        }
        unsafe {
            let expected = avx512_u64x2_block_to_vec(expected_data);
            let v = avx512_u64x2_block_to_vec(value_data);
            let key =
                AVX512Key::<8, 16, 8>::new(PREFIX_U64, [V_U64; 8], SpeckVersion::Speck128_128);
            let mut out = Vec::new();
            avx512_block_compare_u64(&expected, &v, &key, &mut out);
            assert!(out.is_empty());
        }
    }
}
