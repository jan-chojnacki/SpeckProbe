mod scalar;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

mod macros;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub use scalar::search::*;
pub use scalar::validate::*;

#[cfg(target_arch = "aarch64")]
pub use speck_probe::search::executor::backend::aarch64::search::*;

#[cfg(target_arch = "x86_64")]
pub use x86_64::search::*;
