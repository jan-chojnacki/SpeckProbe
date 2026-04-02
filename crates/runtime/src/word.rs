use std::arch::x86_64::{__m128i, __m256i, __m512i};

pub trait EngineWord: Copy + Clone + Send + Sync + 'static {}
impl EngineWord for u16 {}
impl EngineWord for u32 {}
impl EngineWord for u64 {}
impl EngineWord for __m128i {}
impl EngineWord for __m256i {}
impl EngineWord for __m512i {}

pub trait ValidatorWord: Copy + Clone + Send + Sync + PartialEq + 'static {}
impl ValidatorWord for u16 {}
impl ValidatorWord for u32 {}
impl ValidatorWord for u64 {}
