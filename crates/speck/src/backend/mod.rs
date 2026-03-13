mod avx;
mod avx2;
mod avx512;
mod neon;
mod scalar;

#[cfg(target_arch = "x86_64")]
pub use avx::*;
#[cfg(target_arch = "x86_64")]
pub use avx2::*;
#[cfg(target_arch = "x86_64")]
pub use avx512::*;
#[cfg(target_arch = "aarch64")]
pub use neon::*;
pub use scalar::*;
