macro_rules! sse2_ror {
    (16, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_or_si128(
            core::arch::x86_64::_mm_srli_epi16($v, $n),
            core::arch::x86_64::_mm_slli_epi16($v, 16 - $n),
        )
    };
    (24, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_or_si128(
                core::arch::x86_64::_mm_srli_epi32($v, $n),
                core::arch::x86_64::_mm_slli_epi32($v, 24 - $n),
            ),
            core::arch::x86_64::_mm_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_or_si128(
            core::arch::x86_64::_mm_srli_epi32($v, $n),
            core::arch::x86_64::_mm_slli_epi32($v, 32 - $n),
        )
    };
    (48, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_or_si128(
                core::arch::x86_64::_mm_srli_epi64($v, $n),
                core::arch::x86_64::_mm_slli_epi64($v, 48 - $n),
            ),
            core::arch::x86_64::_mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_or_si128(
            core::arch::x86_64::_mm_srli_epi64($v, $n),
            core::arch::x86_64::_mm_slli_epi64($v, 64 - $n),
        )
    };
}

macro_rules! sse2_rol {
    (16, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_or_si128(
            core::arch::x86_64::_mm_slli_epi16($v, $n),
            core::arch::x86_64::_mm_srli_epi16($v, 16 - $n),
        )
    };
    (24, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_or_si128(
                core::arch::x86_64::_mm_slli_epi32($v, $n),
                core::arch::x86_64::_mm_srli_epi32($v, 24 - $n),
            ),
            core::arch::x86_64::_mm_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_or_si128(
            core::arch::x86_64::_mm_slli_epi32($v, $n),
            core::arch::x86_64::_mm_srli_epi32($v, 32 - $n),
        )
    };
    (48, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_or_si128(
                core::arch::x86_64::_mm_slli_epi64($v, $n),
                core::arch::x86_64::_mm_srli_epi64($v, 48 - $n),
            ),
            core::arch::x86_64::_mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm_or_si128(
            core::arch::x86_64::_mm_slli_epi64($v, $n),
            core::arch::x86_64::_mm_srli_epi64($v, 64 - $n),
        )
    };
}

macro_rules! sse2_add {
    (16, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_add_epi16($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_add_epi32($a, $b),
            core::arch::x86_64::_mm_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_add_epi32($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_add_epi64($a, $b),
            core::arch::x86_64::_mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_add_epi64($a, $b)
    };
}

macro_rules! sse2_sub {
    (16, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_sub_epi16($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_sub_epi32($a, $b),
            core::arch::x86_64::_mm_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_sub_epi32($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_sub_epi64($a, $b),
            core::arch::x86_64::_mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_sub_epi64($a, $b)
    };
}

macro_rules! sse2_xor {
    (16, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_xor_si128($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_xor_si128($a, $b),
            core::arch::x86_64::_mm_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_xor_si128($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_and_si128(
            core::arch::x86_64::_mm_xor_si128($a, $b),
            core::arch::x86_64::_mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm_xor_si128($a, $b)
    };
}

macro_rules! sse2_set {
    (16, $n:expr) => {
        core::arch::x86_64::_mm_set1_epi16($n as i16)
    };
    (24, $n:expr) => {
        core::arch::x86_64::_mm_set1_epi32($n as i32)
    };
    (32, $n:expr) => {
        core::arch::x86_64::_mm_set1_epi32($n as i32)
    };
    (48, $n:expr) => {
        core::arch::x86_64::_mm_set1_epi64x($n as i64)
    };
    (64, $n:expr) => {
        core::arch::x86_64::_mm_set1_epi64x($n as i64)
    };
}


pub(crate) use sse2_ror;
pub(crate) use sse2_rol;
pub(crate) use sse2_add;
pub(crate) use sse2_sub;
pub(crate) use sse2_xor;
pub(crate) use sse2_set;