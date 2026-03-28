pub mod scalar;

#[cfg(target_arch = "aarch64")]
pub mod neon;
pub mod search;
pub(crate) mod x86_64;
