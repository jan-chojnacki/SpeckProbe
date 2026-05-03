macro_rules! neon_ror {
    (16, $v:expr, $n:expr) => {
        core::arch::aarch64::vorrq_u16(
            core::arch::aarch64::vshrq_n_u16::<$n>($v),
            core::arch::aarch64::vshlq_n_u16::<{ 16 - $n }>($v),
        )
    };
    (24, $v:expr, $n:expr) => {
        core::arch::aarch64::vandq_u32(
            core::arch::aarch64::vorrq_u32(
                core::arch::aarch64::vshrq_n_u32::<$n>($v),
                core::arch::aarch64::vshlq_n_u32::<{ 24 - $n }>($v),
            ),
            core::arch::aarch64::vdupq_n_u32(0x00FF_FFFF),
        )
    };
    (32, $v:expr, $n:expr) => {
        core::arch::aarch64::vorrq_u32(
            core::arch::aarch64::vshrq_n_u32::<$n>($v),
            core::arch::aarch64::vshlq_n_u32::<{ 32 - $n }>($v),
        )
    };
    (48, $v:expr, $n:expr) => {
        core::arch::aarch64::vandq_u64(
            core::arch::aarch64::vorrq_u64(
                core::arch::aarch64::vshrq_n_u64::<$n>($v),
                core::arch::aarch64::vshlq_n_u64::<{ 48 - $n }>($v),
            ),
            core::arch::aarch64::vdupq_n_u64(0x0000_FFFF_FFFF_FFFF),
        )
    };
    (64, $v:expr, $n:expr) => {
        core::arch::aarch64::vorrq_u64(
            core::arch::aarch64::vshrq_n_u64::<$n>($v),
            core::arch::aarch64::vshlq_n_u64::<{ 64 - $n }>($v),
        )
    };
}

macro_rules! neon_rol {
    (16, $v:expr, $n:expr) => {
        core::arch::aarch64::vorrq_u16(
            core::arch::aarch64::vshlq_n_u16::<$n>($v),
            core::arch::aarch64::vshrq_n_u16::<{ 16 - $n }>($v),
        )
    };
    (24, $v:expr, $n:expr) => {
        core::arch::aarch64::vandq_u32(
            core::arch::aarch64::vorrq_u32(
                core::arch::aarch64::vshlq_n_u32::<$n>($v),
                core::arch::aarch64::vshrq_n_u32::<{ 24 - $n }>($v),
            ),
            core::arch::aarch64::vdupq_n_u32(0x00FF_FFFF),
        )
    };
    (32, $v:expr, $n:expr) => {
        core::arch::aarch64::vorrq_u32(
            core::arch::aarch64::vshlq_n_u32::<$n>($v),
            core::arch::aarch64::vshrq_n_u32::<{ 32 - $n }>($v),
        )
    };
    (48, $v:expr, $n:expr) => {
        core::arch::aarch64::vandq_u64(
            core::arch::aarch64::vorrq_u64(
                core::arch::aarch64::vshlq_n_u64::<$n>($v),
                core::arch::aarch64::vshrq_n_u64::<{ 48 - $n }>($v),
            ),
            core::arch::aarch64::vdupq_n_u64(0x0000_FFFF_FFFF_FFFF),
        )
    };
    (64, $v:expr, $n:expr) => {
        core::arch::aarch64::vorrq_u64(
            core::arch::aarch64::vshlq_n_u64::<$n>($v),
            core::arch::aarch64::vshrq_n_u64::<{ 64 - $n }>($v),
        )
    };
}

macro_rules! neon_add {
    (16, $a:expr, $b:expr) => {
        core::arch::aarch64::vaddq_u16($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::aarch64::vandq_u32(
            core::arch::aarch64::vaddq_u32($a, $b),
            core::arch::aarch64::vdupq_n_u32(0x00FF_FFFF),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::aarch64::vaddq_u32($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::aarch64::vandq_u64(
            core::arch::aarch64::vaddq_u64($a, $b),
            core::arch::aarch64::vdupq_n_u64(0x0000_FFFF_FFFF_FFFF),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::aarch64::vaddq_u64($a, $b)
    };
}

macro_rules! neon_sub {
    (16, $a:expr, $b:expr) => {
        core::arch::aarch64::vsubq_u16($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::aarch64::vandq_u32(
            core::arch::aarch64::vsubq_u32($a, $b),
            core::arch::aarch64::vdupq_n_u32(0x00FF_FFFF),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::aarch64::vsubq_u32($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::aarch64::vandq_u64(
            core::arch::aarch64::vsubq_u64($a, $b),
            core::arch::aarch64::vdupq_n_u64(0x0000_FFFF_FFFF_FFFF),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::aarch64::vsubq_u64($a, $b)
    };
}

macro_rules! neon_xor {
    (16, $a:expr, $b:expr) => {
        core::arch::aarch64::veorq_u16($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::aarch64::vandq_u32(
            core::arch::aarch64::veorq_u32($a, $b),
            core::arch::aarch64::vdupq_n_u32(0x00FF_FFFF),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::aarch64::veorq_u32($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::aarch64::vandq_u64(
            core::arch::aarch64::veorq_u64($a, $b),
            core::arch::aarch64::vdupq_n_u64(0x0000_FFFF_FFFF_FFFF),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::aarch64::veorq_u64($a, $b)
    };
}

macro_rules! neon_set {
    (16, $n:expr) => {
        core::arch::aarch64::vdupq_n_u16($n)
    };
    (24, $n:expr) => {
        core::arch::aarch64::vdupq_n_u32($n)
    };
    (32, $n:expr) => {
        core::arch::aarch64::vdupq_n_u32($n)
    };
    (48, $n:expr) => {
        core::arch::aarch64::vdupq_n_u64($n)
    };
    (64, $n:expr) => {
        core::arch::aarch64::vdupq_n_u64($n)
    };
}

pub(crate) use neon_ror;
pub(crate) use neon_rol;
pub(crate) use neon_add;
pub(crate) use neon_sub;
pub(crate) use neon_xor;
pub(crate) use neon_set;
