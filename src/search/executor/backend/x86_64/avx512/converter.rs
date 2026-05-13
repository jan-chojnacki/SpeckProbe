use std::arch::x86_64::{__m512i, _mm512_set1_epi16, _mm512_set1_epi32, _mm512_set1_epi64};

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx512f` before calling this function."]
pub fn avx512_u16x2_block_to_vec(v: [u16; 2]) -> [__m512i; 2] {
    v.map(|l| _mm512_set1_epi16(l as i16))
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx512f` before calling this function."]
pub fn avx512_u32x2_block_to_vec(v: [u32; 2]) -> [__m512i; 2] {
    v.map(|l| _mm512_set1_epi32(l as i32))
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx512f` before calling this function."]
pub fn avx512_u64x2_block_to_vec(v: [u64; 2]) -> [__m512i; 2] {
    v.map(|l| _mm512_set1_epi64(l as i64))
}
