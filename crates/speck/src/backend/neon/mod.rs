#[cfg(target_arch = "aarch64")]
mod decrypt_block;
#[cfg(target_arch = "aarch64")]
mod decrypt_round;
#[cfg(target_arch = "aarch64")]
mod encrypt_block;
#[cfg(target_arch = "aarch64")]
mod encrypt_round;
#[cfg(target_arch = "aarch64")]
mod expand_key;
#[cfg(target_arch = "aarch64")]
mod operations;

#[cfg(target_arch = "aarch64")]
pub use decrypt_block::*;
#[cfg(target_arch = "aarch64")]
pub use encrypt_block::*;
