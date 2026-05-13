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
