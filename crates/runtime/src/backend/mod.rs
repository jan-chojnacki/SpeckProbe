#[cfg(target_arch = "aarch64")]
pub mod aarch64;
pub mod dispatch;
pub mod scalar;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

mod macros;
mod runner;
