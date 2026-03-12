mod avx;
mod avx2;
mod avx512;
mod neon;
mod scalar;

pub use avx::*;
pub use avx2::*;
pub use avx512::*;
pub use neon::*;
pub use scalar::*;
