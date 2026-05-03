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

pub(super) use decrypt_round::sse2_decrypt_round_inline;
pub(super) use encrypt_round::sse2_encrypt_round_inline;
pub(super) use expand_key::sse2_expand_key_inline;
pub(super) use operations::sse2_add;
pub(super) use operations::sse2_rol;
pub(super) use operations::sse2_ror;
pub(super) use operations::sse2_set;
pub(super) use operations::sse2_sub;
pub(super) use operations::sse2_xor;

#[cfg(target_arch = "x86_64")]
pub use decrypt_block::*;
#[cfg(target_arch = "x86_64")]
pub use encrypt_block::*;
