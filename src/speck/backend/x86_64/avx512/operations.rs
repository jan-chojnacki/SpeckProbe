macro_rules! avx512_ror {
    (16, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_or_si512(
            core::arch::x86_64::_mm512_srli_epi16($v, $n),
            core::arch::x86_64::_mm512_slli_epi16($v, 16 - $n),
        )
    };
    (24, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_or_si512(
                core::arch::x86_64::_mm512_srli_epi32($v, $n),
                core::arch::x86_64::_mm512_slli_epi32($v, 24 - $n),
            ),
            core::arch::x86_64::_mm512_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_ror_epi32($v, $n)
    };
    (48, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_or_si512(
                core::arch::x86_64::_mm512_srli_epi64($v, $n),
                core::arch::x86_64::_mm512_slli_epi64($v, 48 - $n),
            ),
            core::arch::x86_64::_mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_ror_epi64($v, $n)
    };
}

macro_rules! avx512_rol {
    (16, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_or_si512(
            core::arch::x86_64::_mm512_slli_epi16($v, $n),
            core::arch::x86_64::_mm512_srli_epi16($v, 16 - $n),
        )
    };
    (24, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_or_si512(
                core::arch::x86_64::_mm512_slli_epi32($v, $n),
                core::arch::x86_64::_mm512_srli_epi32($v, 24 - $n),
            ),
            core::arch::x86_64::_mm512_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_rol_epi32($v, $n)
    };
    (48, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_or_si512(
                core::arch::x86_64::_mm512_slli_epi64($v, $n),
                core::arch::x86_64::_mm512_srli_epi64($v, 48 - $n),
            ),
            core::arch::x86_64::_mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $v:expr, $n:expr) => {
        core::arch::x86_64::_mm512_rol_epi64($v, $n)
    };
}

macro_rules! avx512_add {
    (16, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_add_epi16($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_add_epi32($a, $b),
            core::arch::x86_64::_mm512_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_add_epi32($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_add_epi64($a, $b),
            core::arch::x86_64::_mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_add_epi64($a, $b)
    };
}

macro_rules! avx512_sub {
    (16, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_sub_epi16($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_sub_epi32($a, $b),
            core::arch::x86_64::_mm512_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_sub_epi32($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_sub_epi64($a, $b),
            core::arch::x86_64::_mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_sub_epi64($a, $b)
    };
}

macro_rules! avx512_xor {
    (16, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_xor_si512($a, $b)
    };
    (24, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_xor_si512($a, $b),
            core::arch::x86_64::_mm512_set1_epi32(0x00FF_FFFFu32 as i32),
        )
    };
    (32, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_xor_si512($a, $b)
    };
    (48, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_and_si512(
            core::arch::x86_64::_mm512_xor_si512($a, $b),
            core::arch::x86_64::_mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64),
        )
    };
    (64, $a:expr, $b:expr) => {
        core::arch::x86_64::_mm512_xor_si512($a, $b)
    };
}

macro_rules! avx512_set {
    (16, $n:expr) => {
        core::arch::x86_64::_mm512_set1_epi16($n as i16)
    };
    (24, $n:expr) => {
        core::arch::x86_64::_mm512_set1_epi32($n as i32)
    };
    (32, $n:expr) => {
        core::arch::x86_64::_mm512_set1_epi32($n as i32)
    };
    (48, $n:expr) => {
        core::arch::x86_64::_mm512_set1_epi64($n as i64)
    };
    (64, $n:expr) => {
        core::arch::x86_64::_mm512_set1_epi64($n as i64)
    };
}

pub(crate) use avx512_add;
pub(crate) use avx512_rol;
pub(crate) use avx512_ror;
pub(crate) use avx512_set;
pub(crate) use avx512_sub;
pub(crate) use avx512_xor;
