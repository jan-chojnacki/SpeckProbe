#[cfg(target_arch = "x86_64")]
pub mod sse2;
#[cfg(target_arch = "x86_64")]
pub mod avx512;
#[cfg(target_arch = "x86_64")]
pub mod avx2;