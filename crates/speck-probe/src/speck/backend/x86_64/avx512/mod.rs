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

pub(super) use decrypt_round::avx512_decrypt_round_inline;
pub(super) use encrypt_round::avx512_encrypt_round_inline;
pub(super) use expand_key::avx512_expand_key_inline;
pub(super) use operations::avx512_add;
pub(super) use operations::avx512_rol;
pub(super) use operations::avx512_ror;
pub(super) use operations::avx512_set;
pub(super) use operations::avx512_sub;
pub(super) use operations::avx512_xor;

#[cfg(target_arch = "x86_64")]
pub use decrypt_block::*;
#[cfg(target_arch = "x86_64")]
pub use encrypt_block::*;
