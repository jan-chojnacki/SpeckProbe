#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{__m128i, __m256i, __m512i};

pub trait EngineWord: Copy + Clone + Send + Sync + 'static {}
impl EngineWord for u16 {}
impl EngineWord for u32 {}
impl EngineWord for u64 {}

#[cfg(target_arch = "x86_64")]
impl EngineWord for __m128i {}

#[cfg(target_arch = "x86_64")]
impl EngineWord for __m256i {}

#[cfg(target_arch = "x86_64")]
impl EngineWord for __m512i {}

#[cfg(target_arch = "aarch64")]
impl EngineWord for uint16x8_t {}

#[cfg(target_arch = "aarch64")]
impl EngineWord for uint32x4_t {}

#[cfg(target_arch = "aarch64")]
impl EngineWord for uint64x2_t {}

pub trait ValidatorWord:
    Copy + Clone + Send + Sync + PartialEq + std::ops::BitXor<Output = Self> + 'static
{
    fn from_u64(v: u64) -> Self;
}
impl ValidatorWord for u16 {
    fn from_u64(v: u64) -> Self {
        v as u16
    }
}
impl ValidatorWord for u32 {
    fn from_u64(v: u64) -> Self {
        v as u32
    }
}
impl ValidatorWord for u64 {
    fn from_u64(v: u64) -> Self {
        v
    }
}
