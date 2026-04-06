#[cfg(target_arch = "aarch64")]
pub mod aarch64;
pub mod dispatch;
pub mod scalar;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod versions {
    #[cfg(target_arch = "aarch64")]
    pub(crate) use crate::backend::aarch64::neon::*;
    pub(crate) use crate::backend::scalar::*;
    #[cfg(target_arch = "x86_64")]
    pub(crate) use crate::backend::x86_64::avx2::*;
    #[cfg(target_arch = "x86_64")]
    pub(crate) use crate::backend::x86_64::avx512::*;
    #[cfg(target_arch = "x86_64")]
    pub(crate) use crate::backend::x86_64::sse2::*;
}

mod macors;
