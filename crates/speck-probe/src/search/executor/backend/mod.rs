#[cfg(target_arch = "aarch64")]
pub mod aarch64;
pub mod scalar;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

mod macros;
mod runner;

#[cfg(target_arch = "x86_64")]
pub use x86_64::avx2::comparator::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::avx512::comparator::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::search::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::sse2::comparator::*;

#[cfg(target_arch = "aarch64")]
pub use aarch64::neon::comparator::*;
#[cfg(target_arch = "aarch64")]
pub use aarch64::search::*;

pub use scalar::comparator::*;
pub use scalar::search::*;
pub use scalar::validate::*;
