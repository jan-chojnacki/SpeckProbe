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

#[cfg(target_arch = "aarch64")]
macro_rules! neon_word_ty {
    (16) => {
        uint16x8_t
    };
    (24) => {
        uint32x4_t
    };
    (32) => {
        uint32x4_t
    };
    (48) => {
        uint64x2_t
    };
    (64) => {
        uint64x2_t
    };
}

#[cfg(target_arch = "aarch64")]
pub(crate) use neon_word_ty;
