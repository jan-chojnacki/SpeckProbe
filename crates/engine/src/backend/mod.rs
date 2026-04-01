mod scalar;

#[cfg(target_arch = "aarch64")]
mod neon;
mod search;
pub(crate) mod x86_64;

pub use search::*;
pub use x86_64::search::*;
