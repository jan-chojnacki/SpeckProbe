#[cfg(target_arch = "x86_64")]
mod decrypt_block;
#[cfg(target_arch = "x86_64")]
mod decrypt_round;
#[cfg(target_arch = "x86_64")]
mod encrypt_block;
#[cfg(target_arch = "x86_64")]
mod encrypt_round;
#[cfg(target_arch = "x86_64")]
mod expand_key;
#[cfg(target_arch = "x86_64")]
mod operations;

pub(super) use decrypt_round::avx2_decrypt_round_inline;
pub(super) use encrypt_round::avx2_encrypt_round_inline;
pub(super) use expand_key::avx2_expand_key_inline;
pub(super) use operations::avx2_add;
pub(super) use operations::avx2_rol;
pub(super) use operations::avx2_ror;
pub(super) use operations::avx2_set;
pub(super) use operations::avx2_sub;
pub(super) use operations::avx2_xor;

#[cfg(target_arch = "x86_64")]
pub use decrypt_block::*;
#[cfg(target_arch = "x86_64")]
pub use encrypt_block::*;
