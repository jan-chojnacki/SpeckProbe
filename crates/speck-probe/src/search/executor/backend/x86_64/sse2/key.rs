use crate::search::executor::backend::macros::impl_x86_simd_key;
use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_setzero_si128};

impl_x86_simd_key!(
    SSE2Key,
    16,
    "sse2",
    __m128i,
    128,
    _mm_setzero_si128,
    _mm_load_si128
);
