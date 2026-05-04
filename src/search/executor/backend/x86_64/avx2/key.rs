use crate::search::executor::backend::macros::impl_x86_simd_key;
use std::arch::x86_64::{__m256i, _mm256_load_si256, _mm256_setzero_si256};

impl_x86_simd_key!(
    AVX2Key,
    32,
    "avx2",
    __m256i,
    256,
    _mm256_setzero_si256,
    _mm256_load_si256
);
