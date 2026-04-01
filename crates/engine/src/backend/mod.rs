mod scalar;

#[cfg(target_arch = "aarch64")]
mod neon;
pub(crate) mod x86_64;

pub use scalar::search::*;
pub use scalar::validate::*;
pub use x86_64::search::*;
