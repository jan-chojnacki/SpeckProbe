pub mod aarch64;
mod adapter;
pub mod key_idx;
mod key_words_inline;
mod scalar;
mod u24;
mod u48;
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::neon::*;
pub use scalar::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::avx2::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::avx512::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::sse2::*;

pub(super) use adapter::impl_adapter;
pub(super) use key_idx::key_idx;
pub(super) use key_words_inline::key_words_inline;

pub(super) use u24::U24;
pub(super) use u48::U48;
