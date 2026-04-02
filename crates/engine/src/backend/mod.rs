mod scalar;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub use scalar::search::*;
pub use scalar::validate::*;

#[cfg(target_arch = "aarch64")]
pub use aarch64::search::*;

#[cfg(target_arch = "x86_64")]
pub use x86_64::search::*;
