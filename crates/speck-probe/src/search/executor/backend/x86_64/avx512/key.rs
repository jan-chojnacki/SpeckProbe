use crate::search::executor::backend::macros::impl_x86_simd_key;
use std::arch::x86_64::{__m512i, _mm512_load_si512, _mm512_setzero_si512};

impl_x86_simd_key!(
    AVX512Key,
    64,
    "avx512bw",
    __m512i,
    512,
    _mm512_setzero_si512,
    _mm512_load_si512
);
