use std::arch::x86_64::{__m256i, _mm256_set1_epi16, _mm256_set1_epi32, _mm256_set1_epi64x};

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
pub fn avx2_u16x2_block_to_vec(v: [u16; 2]) -> [__m256i; 2] {
    v.map(|l| _mm256_set1_epi16(l as i16))
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
pub fn avx2_u32x2_block_to_vec(v: [u32; 2]) -> [__m256i; 2] {
    v.map(|l| _mm256_set1_epi32(l as i32))
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
pub fn avx2_u64x2_block_to_vec(v: [u64; 2]) -> [__m256i; 2] {
    v.map(|l| _mm256_set1_epi64x(l as i64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([0x1234u16, 0x5678u16], [0x1234u16, 0x5678u16])]
    #[case([0u16, 0u16],           [0u16, 0u16])]
    #[case([u16::MAX, u16::MAX],   [u16::MAX, u16::MAX])]
    fn avx2_u16x2_broadcast(#[case] input: [u16; 2], #[case] expected: [u16; 2]) {
        if !is_x86_feature_detected!("avx2") {
            return;
        }
        unsafe {
            let [va, vb] = avx2_u16x2_block_to_vec(input);
            let a: [u16; 16] = std::mem::transmute(va);
            let b: [u16; 16] = std::mem::transmute(vb);
            assert!(a.iter().all(|&x| x == expected[0]));
            assert!(b.iter().all(|&x| x == expected[1]));
        }
    }

    #[rstest]
    #[case([0x1234_5678u32, 0x9abc_def0u32], [0x1234_5678u32, 0x9abc_def0u32])]
    #[case([0u32, 0u32],                     [0u32, 0u32])]
    #[case([u32::MAX, u32::MAX],             [u32::MAX, u32::MAX])]
    fn avx2_u32x2_broadcast(#[case] input: [u32; 2], #[case] expected: [u32; 2]) {
        if !is_x86_feature_detected!("avx2") {
            return;
        }
        unsafe {
            let [va, vb] = avx2_u32x2_block_to_vec(input);
            let a: [u32; 8] = std::mem::transmute(va);
            let b: [u32; 8] = std::mem::transmute(vb);
            assert!(a.iter().all(|&x| x == expected[0]));
            assert!(b.iter().all(|&x| x == expected[1]));
        }
    }

    #[rstest]
    #[case([0x0123_4567_89ab_cdefu64, 0xfedc_ba98_7654_3210u64], [0x0123_4567_89ab_cdefu64, 0xfedc_ba98_7654_3210u64])]
    #[case([0u64, 0u64],                                         [0u64, 0u64])]
    #[case([u64::MAX, u64::MAX],                                 [u64::MAX, u64::MAX])]
    fn avx2_u64x2_broadcast(#[case] input: [u64; 2], #[case] expected: [u64; 2]) {
        if !is_x86_feature_detected!("avx2") {
            return;
        }
        unsafe {
            let [va, vb] = avx2_u64x2_block_to_vec(input);
            let a: [u64; 4] = std::mem::transmute(va);
            let b: [u64; 4] = std::mem::transmute(vb);
            assert!(a.iter().all(|&x| x == expected[0]));
            assert!(b.iter().all(|&x| x == expected[1]));
        }
    }
}
