use std::arch::x86_64::{__m128i, _mm_set1_epi16, _mm_set1_epi32, _mm_set1_epi64x};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
pub fn sse2_u16x2_block_to_vec(v: [u16; 2]) -> [__m128i; 2] {
    v.map(|l| _mm_set1_epi16(l as i16))
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
pub fn sse2_u32x2_block_to_vec(v: [u32; 2]) -> [__m128i; 2] {
    v.map(|l| _mm_set1_epi32(l as i32))
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
pub fn sse2_u64x2_block_to_vec(v: [u64; 2]) -> [__m128i; 2] {
    v.map(|l| _mm_set1_epi64x(l as i64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([0x1234u16, 0x5678u16], [0x1234u16, 0x5678u16])]
    #[case([0u16, 0u16],           [0u16, 0u16])]
    #[case([u16::MAX, u16::MAX],   [u16::MAX, u16::MAX])]
    fn sse2_u16x2_broadcast(#[case] input: [u16; 2], #[case] expected: [u16; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let [va, vb] = sse2_u16x2_block_to_vec(input);
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
    fn sse2_u32x2_broadcast(#[case] input: [u32; 2], #[case] expected: [u32; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let [va, vb] = sse2_u32x2_block_to_vec(input);
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
    fn sse2_u64x2_broadcast(#[case] input: [u64; 2], #[case] expected: [u64; 2]) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let [va, vb] = sse2_u64x2_block_to_vec(input);
            let a: [u64; 2] = std::mem::transmute(va);
            let b: [u64; 2] = std::mem::transmute(vb);
            assert!(a.iter().all(|&x| x == expected[0]));
            assert!(b.iter().all(|&x| x == expected[1]));
        }
    }
}
