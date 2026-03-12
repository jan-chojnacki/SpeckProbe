mod decrypt_block;
mod decrypt_round;
mod encrypt_block;
mod encrypt_round;
mod expand_key;
mod operations;

pub use decrypt_block::*;
pub use encrypt_block::*;

use core::arch::aarch64::*;

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

pub(crate) use neon_word_ty;
