use crate::aarch64::neon::key::NEONKey;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{
    uint16x8_t, uint32x4_t, uint64x2_t, vandq_u16, vandq_u32, vandq_u64, vceqq_u16, vceqq_u32,
    vceqq_u64, vst1q_u16, vst1q_u32, vst1q_u64,
};

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_block_compare_u16<const BYTES: usize, const PREFIX: usize>(
    e: &[uint16x8_t; 2],
    v: &[uint16x8_t; 2],
    key: &NEONKey<8, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = vceqq_u16(e[0], v[0]);
    let cmp_hi = vceqq_u16(e[1], v[1]);
    let cmp = vandq_u16(cmp_lo, cmp_hi);

    let mut lanes = [0u16; 8];
    unsafe {
        vst1q_u16(lanes.as_mut_ptr(), cmp);
    }
    for (i, m) in lanes.into_iter().enumerate() {
        if m == u16::MAX {
            out.push(key.get(i));
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_block_compare_u32<const BYTES: usize, const PREFIX: usize>(
    e: &[uint32x4_t; 2],
    v: &[uint32x4_t; 2],
    key: &NEONKey<4, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = vceqq_u32(e[0], v[0]);
    let cmp_hi = vceqq_u32(e[1], v[1]);
    let cmp = vandq_u32(cmp_lo, cmp_hi);

    let mut lanes = [0u32; 4];
    unsafe {
        vst1q_u32(lanes.as_mut_ptr(), cmp);
    }
    for (i, m) in lanes.into_iter().enumerate() {
        if m == u32::MAX {
            out.push(key.get(i));
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_block_compare_u64<const BYTES: usize, const PREFIX: usize>(
    e: &[uint64x2_t; 2],
    v: &[uint64x2_t; 2],
    key: &NEONKey<2, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = vceqq_u64(e[0], v[0]);
    let cmp_hi = vceqq_u64(e[1], v[1]);
    let cmp = vandq_u64(cmp_lo, cmp_hi);

    let mut lanes = [0u64; 2];
    unsafe {
        vst1q_u64(lanes.as_mut_ptr(), cmp);
    }
    for (i, m) in lanes.into_iter().enumerate() {
        if m == u64::MAX {
            out.push(key.get(i));
        }
    }
}

#[cfg(all(test, target_arch = "aarch64"))]
mod tests {
    use super::*;
    use crate::aarch64::neon::converter::{
        neon_u16x2_block_to_vec, neon_u32x2_block_to_vec, neon_u64x2_block_to_vec,
    };
    use rstest::rstest;
    use speck::SpeckVersion;

    const KEY_U16: &[u8; 8] = &[0x18, 0x19, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01];
    const V_U16: u64 = 0x0100_0908_1110_1918;

    #[rstest]
    #[case([0xa868u16, 0x42f2])]
    fn neon_u16_hit(#[case] expected_data: [u16; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let expected = neon_u16x2_block_to_vec(expected_data);
            let key = NEONKey::<8, 8, 0>::new(&[], [V_U16; 8], SpeckVersion::Speck32_64);
            let mut out = Vec::new();
            neon_block_compare_u16(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 8);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U16.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0xa868u16, 0x42f2], [0x0000u16, 0x0000])]
    #[case([0xa868u16, 0x42f2], [0xa868u16, 0x0000])]
    fn neon_u16_miss(#[case] expected_data: [u16; 2], #[case] value_data: [u16; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let expected = neon_u16x2_block_to_vec(expected_data);
            let v = neon_u16x2_block_to_vec(value_data);
            let key = NEONKey::<8, 8, 0>::new(&[], [V_U16; 8], SpeckVersion::Speck32_64);
            let mut out = Vec::new();
            neon_block_compare_u16(&expected, &v, &key, &mut out);
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
    fn neon_u32_hit(#[case] expected_data: [u32; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let expected = neon_u32x2_block_to_vec(expected_data);
            let key = NEONKey::<4, 12, 4>::new(PREFIX_U32, [V_U32; 4], SpeckVersion::Speck64_96);
            let mut out = Vec::new();
            neon_block_compare_u32(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 4);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U32.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0x9f7952ecu32, 0x4175946c], [0x00000000u32, 0x00000000])]
    #[case([0x9f7952ecu32, 0x4175946c], [0x9f7952ecu32, 0x00000000])]
    fn neon_u32_miss(#[case] expected_data: [u32; 2], #[case] value_data: [u32; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let expected = neon_u32x2_block_to_vec(expected_data);
            let v = neon_u32x2_block_to_vec(value_data);
            let key = NEONKey::<4, 12, 4>::new(PREFIX_U32, [V_U32; 4], SpeckVersion::Speck64_96);
            let mut out = Vec::new();
            neon_block_compare_u32(&expected, &v, &key, &mut out);
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
    fn neon_u64_hit(#[case] expected_data: [u64; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let expected = neon_u64x2_block_to_vec(expected_data);
            let key = NEONKey::<2, 16, 8>::new(PREFIX_U64, [V_U64; 2], SpeckVersion::Speck128_128);
            let mut out = Vec::new();
            neon_block_compare_u64(&expected, &expected, &key, &mut out);
            assert_eq!(out.len(), 2);
            for k in &out {
                assert_eq!(k.as_bytes(), KEY_U64.as_slice());
            }
        }
    }

    #[rstest]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18], [0x0u64, 0x0])]
    #[case([0xa65d985179783265u64, 0x7860fedf5c570d18], [0xa65d985179783265u64, 0x0])]
    fn neon_u64_miss(#[case] expected_data: [u64; 2], #[case] value_data: [u64; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let expected = neon_u64x2_block_to_vec(expected_data);
            let v = neon_u64x2_block_to_vec(value_data);
            let key = NEONKey::<2, 16, 8>::new(PREFIX_U64, [V_U64; 2], SpeckVersion::Speck128_128);
            let mut out = Vec::new();
            neon_block_compare_u64(&expected, &v, &key, &mut out);
            assert!(out.is_empty());
        }
    }
}
