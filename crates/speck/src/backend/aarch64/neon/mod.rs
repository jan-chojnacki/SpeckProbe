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

pub(super) use decrypt_round::neon_decrypt_round_inline;
pub(super) use encrypt_round::neon_encrypt_round_inline;
pub(super) use expand_key::neon_expand_key_inline;
pub(super) use operations::neon_add;
pub(super) use operations::neon_rol;
pub(super) use operations::neon_ror;
pub(super) use operations::neon_set;
pub(super) use operations::neon_sub;
pub(super) use operations::neon_xor;

#[cfg(target_arch = "aarch64")]
pub use decrypt_block::*;
#[cfg(target_arch = "aarch64")]
pub use encrypt_block::*;
