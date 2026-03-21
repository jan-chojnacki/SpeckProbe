use std::arch::x86_64::{__m128i, _mm_set1_epi16, _mm_set1_epi32, _mm_set1_epi64x};

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u16x2_block_to_avx_vec(v: [u16; 2]) -> [__m128i; 2] {
    v.map(|l| _mm_set1_epi16(l as i16))
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u32x2_block_to_avx_vec(v: [u32; 2]) -> [__m128i; 2] {
    v.map(|l| _mm_set1_epi32(l as i32))
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u64x2_block_to_avx_vec(v: [u64; 2]) -> [__m128i; 2] {
    v.map(|l| _mm_set1_epi64x(l as i64))
}
