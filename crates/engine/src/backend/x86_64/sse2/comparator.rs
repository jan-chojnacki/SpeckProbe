use crate::backend::x86_64::sse2::key::SSE2Key;
use crate::domain::key::Key;
use std::arch::x86_64::{__m128i, _mm_cmpeq_epi16, _mm_cmpeq_epi32, _mm_movemask_epi8};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
pub fn sse2_block_compare_u16<const BYTES: usize, const PREFIX: usize>(
    e: &[__m128i; 2],
    v: &[__m128i; 2],
    key: &SSE2Key<8, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm_cmpeq_epi16(e[0], v[0]);
    let cmp_hi = _mm_cmpeq_epi16(e[1], v[1]);

    let m_lo = _mm_movemask_epi8(cmp_lo) as u32;
    let m_hi = _mm_movemask_epi8(cmp_hi) as u32;

    let lanes_lo = m_lo & (m_lo >> 1);
    let lanes_hi = m_hi & (m_hi >> 1);

    let lane_bits_lo = ((lanes_lo >> 0) & 0x1)
        | ((lanes_lo >> 1) & 0x2)
        | ((lanes_lo >> 2) & 0x4)
        | ((lanes_lo >> 3) & 0x8)
        | ((lanes_lo >> 4) & 0x10)
        | ((lanes_lo >> 5) & 0x20)
        | ((lanes_lo >> 6) & 0x40)
        | ((lanes_lo >> 7) & 0x80);
    let lane_bits_hi = ((lanes_hi >> 0) & 0x1)
        | ((lanes_hi >> 1) & 0x2)
        | ((lanes_hi >> 2) & 0x4)
        | ((lanes_hi >> 3) & 0x8)
        | ((lanes_hi >> 4) & 0x10)
        | ((lanes_hi >> 5) & 0x20)
        | ((lanes_hi >> 6) & 0x40)
        | ((lanes_hi >> 7) & 0x80);

    let mut lanes = (lane_bits_lo & lane_bits_hi) & 0x00FF;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
pub fn sse2_block_compare_u32<const BYTES: usize, const PREFIX: usize>(
    e: &[__m128i; 2],
    v: &[__m128i; 2],
    key: &SSE2Key<4, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm_cmpeq_epi32(e[0], v[0]);
    let cmp_hi = _mm_cmpeq_epi32(e[1], v[1]);

    let m_lo = _mm_movemask_epi8(cmp_lo) as u32;
    let m_hi = _mm_movemask_epi8(cmp_hi) as u32;

    let lanes_lo = m_lo & (m_lo >> 1) & (m_lo >> 2) & (m_lo >> 3);
    let lanes_hi = m_hi & (m_hi >> 1) & (m_hi >> 2) & (m_hi >> 3);

    let lane_bits_lo = ((lanes_lo >> 0) & 0x1)
        | ((lanes_lo >> 3) & 0x2)
        | ((lanes_lo >> 6) & 0x4)
        | ((lanes_lo >> 9) & 0x8);
    let lane_bits_hi = ((lanes_hi >> 0) & 0x1)
        | ((lanes_hi >> 3) & 0x2)
        | ((lanes_hi >> 6) & 0x4)
        | ((lanes_hi >> 9) & 0x8);

    let mut lanes = (lane_bits_lo & lane_bits_hi) & 0x0F;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
pub fn sse2_block_compare_u64<const BYTES: usize, const PREFIX: usize>(
    e: &[__m128i; 2],
    v: &[__m128i; 2],
    key: &SSE2Key<2, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm_cmpeq_epi32(e[0], v[0]);
    let cmp_hi = _mm_cmpeq_epi32(e[1], v[1]);

    let m_lo = _mm_movemask_epi8(cmp_lo) as u32;
    let m_hi = _mm_movemask_epi8(cmp_hi) as u32;

    let lanes_lo = m_lo
        & (m_lo >> 1)
        & (m_lo >> 2)
        & (m_lo >> 3)
        & (m_lo >> 4)
        & (m_lo >> 5)
        & (m_lo >> 6)
        & (m_lo >> 7);
    let lanes_hi = m_hi
        & (m_hi >> 1)
        & (m_hi >> 2)
        & (m_hi >> 3)
        & (m_hi >> 4)
        & (m_hi >> 5)
        & (m_hi >> 6)
        & (m_hi >> 7);

    let lane_bits_lo = (lanes_lo & 0x1) | ((lanes_lo >> 7) & 0x2);
    let lane_bits_hi = (lanes_hi & 0x1) | ((lanes_hi >> 7) & 0x2);

    let mut lanes = (lane_bits_lo & lane_bits_hi) & 0x03;

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
    use crate::backend::x86_64::sse2::converter::{
        sse2_u16x2_block_to_vec, sse2_u32x2_block_to_vec, sse2_u64x2_block_to_vec,
    };
    use rstest::rstest;
    use speck::SpeckVersion;

    const KEY_U16: &[u8; 8] = &[0x18, 0x19, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01];
    const V_U16: u64 = 0x0100_0908_1110_1918;

    #[rstest]
    #[case([0xa868u16, 0x42f2])]
    fn sse2_u16_hit(#[case] expected_data: [u16; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let expected = sse2_u16x2_block_to_vec(expected_data);
            let key = SSE2Key::<8, 8, 0>::new(&[], [V_U16; 8], SpeckVersion::Speck32_64);
            let mut out = Vec::new();
            sse2_block_compare_u16(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 8);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U16.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0xa868u16, 0x42f2], [0x0000u16, 0x0000])]
    #[case([0xa868u16, 0x42f2], [0xa868u16, 0x0000])]
    fn sse2_u16_miss(#[case] expected_data: [u16; 2], #[case] value_data: [u16; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let expected = sse2_u16x2_block_to_vec(expected_data);
            let v = sse2_u16x2_block_to_vec(value_data);
            let key = SSE2Key::<8, 8, 0>::new(&[], [V_U16; 8], SpeckVersion::Speck32_64);
            let mut out = Vec::new();
            sse2_block_compare_u16(&expected, &v, &key, &mut out);
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
    fn sse2_u32_hit(#[case] expected_data: [u32; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let expected = sse2_u32x2_block_to_vec(expected_data);
            let key = SSE2Key::<4, 12, 4>::new(PREFIX_U32, [V_U32; 4], SpeckVersion::Speck64_96);
            let mut out = Vec::new();
            sse2_block_compare_u32(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 4);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U32.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0x9f7952ecu32, 0x4175946c], [0x00000000u32, 0x00000000])]
    #[case([0x9f7952ecu32, 0x4175946c], [0x9f7952ecu32, 0x00000000])]
    fn sse2_u32_miss(#[case] expected_data: [u32; 2], #[case] value_data: [u32; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let expected = sse2_u32x2_block_to_vec(expected_data);
            let v = sse2_u32x2_block_to_vec(value_data);
            let key = SSE2Key::<4, 12, 4>::new(PREFIX_U32, [V_U32; 4], SpeckVersion::Speck64_96);
            let mut out = Vec::new();
            sse2_block_compare_u32(&expected, &v, &key, &mut out);
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
    fn sse2_u64_hit(#[case] expected_data: [u64; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let expected = sse2_u64x2_block_to_vec(expected_data);
            let key = SSE2Key::<2, 16, 8>::new(PREFIX_U64, [V_U64; 2], SpeckVersion::Speck128_128);
            let mut out = Vec::new();
            sse2_block_compare_u64(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 2);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U64.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18], [0x0u64, 0x0])]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18], [0xa65d985179783265u64, 0x0])]
    fn sse2_u64_miss(#[case] expected_data: [u64; 2], #[case] value_data: [u64; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let expected = sse2_u64x2_block_to_vec(expected_data);
            let v = sse2_u64x2_block_to_vec(value_data);
            let key = SSE2Key::<2, 16, 8>::new(PREFIX_U64, [V_U64; 2], SpeckVersion::Speck128_128);
            let mut out = Vec::new();
            sse2_block_compare_u64(&expected, &v, &key, &mut out);
            assert!(out.is_empty());
        }
    }
}
