use std::arch::x86_64::{__m128i, _mm_loadu_si128};

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u16x4_key_to_avx_vec(v: [[[u8; 2]; 8]; 4]) -> [__m128i; 4] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u24x3_key_to_avx_vec(v: [[[u8; 4]; 4]; 3]) -> [__m128i; 3] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u24x4_key_to_avx_vec(v: [[[u8; 4]; 4]; 4]) -> [__m128i; 4] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u32x3_key_to_avx_vec(v: [[[u8; 4]; 4]; 3]) -> [__m128i; 3] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u32x4_key_to_avx_vec(v: [[[u8; 4]; 4]; 4]) -> [__m128i; 4] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u48x2_key_to_avx_vec(v: [[[u8; 8]; 2]; 2]) -> [__m128i; 2] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u48x3_key_to_avx_vec(v: [[[u8; 8]; 2]; 3]) -> [__m128i; 3] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u64x2_key_to_avx_vec(v: [[[u8; 8]; 2]; 2]) -> [__m128i; 2] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u64x3_key_to_avx_vec(v: [[[u8; 8]; 2]; 3]) -> [__m128i; 3] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn u64x4_key_to_avx_vec(v: [[[u8; 8]; 2]; 4]) -> [__m128i; 4] {
    v.map(|l| unsafe { _mm_loadu_si128(l.as_ptr().cast()) })
}
