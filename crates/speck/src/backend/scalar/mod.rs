mod decrypt_block;
mod decrypt_round;
mod encrypt_block;
mod encrypt_round;
mod expand_key;
mod operations;

macro_rules! word_ty {
    (16) => {
        u16
    };
    (24) => {
        u32
    };
    (32) => {
        u32
    };
    (48) => {
        u64
    };
    (64) => {
        u64
    };
}

use word_ty;

pub use decrypt_block::*;
pub use encrypt_block::*;
pub use operations::U24;
pub use operations::U48;
