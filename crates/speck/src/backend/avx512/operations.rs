use paste::paste;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{
    __m512i, _mm512_add_epi16, _mm512_add_epi32, _mm512_add_epi64, _mm512_and_si512,
    _mm512_or_si512, _mm512_rol_epi32, _mm512_rol_epi64, _mm512_ror_epi32, _mm512_ror_epi64,
    _mm512_set1_epi32, _mm512_set1_epi64, _mm512_slli_epi16, _mm512_srli_epi16, _mm512_sub_epi16,
    _mm512_sub_epi32, _mm512_sub_epi64, _mm512_xor_si512,
};

macro_rules! define_avx512_ror {
    (16, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
            #[target_feature(enable = "avx512bw")]
            pub fn [<avx512_ror_ $n _u16>](v: __m512i) -> __m512i {
                _mm512_or_si512(_mm512_srli_epi16(v, $n), _mm512_slli_epi16(v, 24 - $n))
            }
        }
    };

    (24, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn [<avx512_ror_ $n _u24>](v: __m512i) -> __m512i {
                let r = _mm512_ror_epi32(v, $n);
                _mm512_and_si512(r, _mm512_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn [<avx512_ror_ $n _u48>](v: __m512i) -> __m512i {
                let r = _mm512_ror_epi64(v, $n);
                _mm512_and_si512(r, _mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn [<avx512_ror_ $n _u $word>](v: __m512i) -> __m512i {
                [<_mm512_ror_epi $word>](v, $n)
            }
        }
    };
}

define_avx512_ror!(16, 7);
define_avx512_ror!(16, 2);
define_avx512_ror!(24, 8);
define_avx512_ror!(24, 3);
define_avx512_ror!(32, 8);
define_avx512_ror!(32, 3);
define_avx512_ror!(48, 8);
define_avx512_ror!(48, 3);
define_avx512_ror!(64, 8);
define_avx512_ror!(64, 3);

macro_rules! define_avx512_rol {
    (16, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
            #[target_feature(enable = "avx512bw")]
            pub fn [<avx512_rol_ $n _u16>](v: __m512i) -> __m512i {
                _mm512_or_si512(_mm512_slli_epi16(v, $n), _mm512_srli_epi16(v, 24 - $n))
            }
        }
    };

    (24, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn [<avx512_rol_ $n _u24>](v: __m512i) -> __m512i {
                let r = _mm512_rol_epi32(v, $n);
                _mm512_and_si512(r, _mm512_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn [<avx512_rol_ $n _u48>](v: __m512i) -> __m512i {
                let r = _mm512_rol_epi64(v, $n);
                _mm512_and_si512(r, _mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn [<avx512_rol_ $n _u $word>](v: __m512i) -> __m512i {
                [<_mm512_rol_epi $word>](v, $n)
            }
        }
    };
}

define_avx512_rol!(16, 7);
define_avx512_rol!(24, 8);
define_avx512_rol!(32, 8);
define_avx512_rol!(48, 8);
define_avx512_rol!(64, 8);

macro_rules! define_avx512_add {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn avx512_add_u24(a: __m512i, b: __m512i) -> __m512i {
                let s = _mm512_add_epi32(a, b);
                _mm512_and_si512(s, _mm512_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn avx512_add_u48(a: __m512i, b: __m512i) -> __m512i {
                let s = _mm512_add_epi64(a, b);
                _mm512_and_si512(s, _mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal, $feature:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = $feature))]
            #[target_feature(enable = $feature)]
            pub fn [<avx512_add_u $word>](a: __m512i, b: __m512i) -> __m512i {
                [<_mm512_add_epi $word>](a, b)
            }
        }
    };
}

define_avx512_add!(16, "avx512bw");
define_avx512_add!(24);
define_avx512_add!(32, "avx512f");
define_avx512_add!(48);
define_avx512_add!(64, "avx512f");

macro_rules! define_avx512_sub {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn avx512_sub_u24(a: __m512i, b: __m512i) -> __m512i {
                let s = _mm512_sub_epi32(a, b);
                _mm512_and_si512(s, _mm512_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn avx512_sub_u48(a: __m512i, b: __m512i) -> __m512i {
                let s = _mm512_sub_epi64(a, b);
                _mm512_and_si512(s, _mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal, $feature:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = $feature))]
            #[target_feature(enable = $feature)]
            pub fn [<avx512_sub_u $word>](a: __m512i, b: __m512i) -> __m512i {
                [<_mm512_sub_epi $word>](a, b)
            }
        }
    };
}

define_avx512_sub!(16, "avx512bw");
define_avx512_sub!(24);
define_avx512_sub!(32, "avx512f");
define_avx512_sub!(48);
define_avx512_sub!(64, "avx512f");

macro_rules! define_avx512_xor {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn avx512_xor_u24(a: __m512i, b: __m512i) -> __m512i {
                _mm512_and_si512(_mm512_xor_si512(a, b), _mm512_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn avx512_xor_u48(a: __m512i, b: __m512i) -> __m512i {
                _mm512_and_si512(_mm512_xor_si512(a, b), _mm512_set1_epi64(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
            #[target_feature(enable = "avx512f")]
            pub fn [<avx512_xor_u $word>](a: __m512i, b: __m512i) -> __m512i {
                _mm512_xor_si512(a, b)
            }
        }
    };
}

define_avx512_xor!(16);
define_avx512_xor!(24);
define_avx512_xor!(32);
define_avx512_xor!(48);
define_avx512_xor!(64);
