#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{
    uint16x8_t, uint32x4_t, uint64x2_t, vdupq_n_u16, vdupq_n_u32, vdupq_n_u64,
};

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_u16x2_block_to_vec(v: [u16; 2]) -> [uint16x8_t; 2] {
    v.map(|l| vdupq_n_u16(l))
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_u32x2_block_to_vec(v: [u32; 2]) -> [uint32x4_t; 2] {
    v.map(|l| vdupq_n_u32(l))
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_u64x2_block_to_vec(v: [u64; 2]) -> [uint64x2_t; 2] {
    v.map(|l| vdupq_n_u64(l))
}

#[cfg(all(test, target_arch = "aarch64"))]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([0x1234u16, 0x5678u16], [0x1234u16, 0x5678u16])]
    #[case([0u16, 0u16],           [0u16, 0u16])]
    #[case([u16::MAX, u16::MAX],   [u16::MAX, u16::MAX])]
    fn neon_u16x2_broadcast(#[case] input: [u16; 2], #[case] expected: [u16; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let [va, vb] = neon_u16x2_block_to_vec(input);
            let a: [u16; 8] = std::mem::transmute(va);
            let b: [u16; 8] = std::mem::transmute(vb);
            assert!(a.iter().all(|&x| x == expected[0]));
            assert!(b.iter().all(|&x| x == expected[1]));
        }
    }

    #[rstest]
    #[case([0x1234_5678u32, 0x9abc_def0u32], [0x1234_5678u32, 0x9abc_def0u32])]
    #[case([0u32, 0u32],                     [0u32, 0u32])]
    #[case([u32::MAX, u32::MAX],             [u32::MAX, u32::MAX])]
    fn neon_u32x2_broadcast(#[case] input: [u32; 2], #[case] expected: [u32; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let [va, vb] = neon_u32x2_block_to_vec(input);
            let a: [u32; 4] = std::mem::transmute(va);
            let b: [u32; 4] = std::mem::transmute(vb);
            assert!(a.iter().all(|&x| x == expected[0]));
            assert!(b.iter().all(|&x| x == expected[1]));
        }
    }

    #[rstest]
    #[case([0x0123_4567_89ab_cdefu64, 0xfedc_ba98_7654_3210u64], [0x0123_4567_89ab_cdefu64, 0xfedc_ba98_7654_3210u64])]
    #[case([0u64, 0u64],                                         [0u64, 0u64])]
    #[case([u64::MAX, u64::MAX],                                 [u64::MAX, u64::MAX])]
    fn neon_u64x2_broadcast(#[case] input: [u64; 2], #[case] expected: [u64; 2]) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let [va, vb] = neon_u64x2_block_to_vec(input);
            let a: [u64; 2] = std::mem::transmute(va);
            let b: [u64; 2] = std::mem::transmute(vb);
            assert!(a.iter().all(|&x| x == expected[0]));
            assert!(b.iter().all(|&x| x == expected[1]));
        }
    }
}
