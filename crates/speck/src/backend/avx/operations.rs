use paste::paste;
use std::arch::x86_64::{
    __m128i, _mm_add_epi16, _mm_add_epi32, _mm_add_epi64, _mm_and_si128, _mm_or_si128,
    _mm_set1_epi32, _mm_set1_epi64x, _mm_slli_epi16, _mm_slli_epi32, _mm_slli_epi64,
    _mm_srli_epi16, _mm_srli_epi32, _mm_srli_epi64, _mm_sub_epi16, _mm_sub_epi32, _mm_sub_epi64,
    _mm_xor_si128,
};

macro_rules! define_avx_ror {
    (24, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_ror_ $n _u24>](v: __m128i) -> __m128i {
                let r = _mm_or_si128(
                    _mm_srli_epi32(v, $n),
                    _mm_slli_epi32(v, 24 - $n),
                );
                _mm_and_si128(r, _mm_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_ror_ $n _u48>](v: __m128i) -> __m128i {
                let r = _mm_or_si128(
                    _mm_srli_epi64(v, $n),
                    _mm_slli_epi64(v, 48 - $n),
                );
                _mm_and_si128(r, _mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_ror_ $n _u $word>](v: __m128i) -> __m128i {
                _mm_or_si128([<_mm_srli_epi $word>](v, $n), [<_mm_slli_epi $word>](v, $word - $n))
            }
        }
    };
}

define_avx_ror!(16, 7);
define_avx_ror!(16, 2);
define_avx_ror!(24, 8);
define_avx_ror!(24, 3);
define_avx_ror!(32, 8);
define_avx_ror!(32, 3);
define_avx_ror!(48, 8);
define_avx_ror!(48, 3);
define_avx_ror!(64, 8);
define_avx_ror!(64, 3);

macro_rules! define_avx_rol {
    (24, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_rol_ $n _u24>](v: __m128i) -> __m128i {
                let r = _mm_or_si128(
                    _mm_slli_epi32(v, $n),
                    _mm_srli_epi32(v, 24 - $n),
                );
                _mm_and_si128(r, _mm_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_rol_ $n _u48>](v: __m128i) -> __m128i {
                let r = _mm_or_si128(
                    _mm_slli_epi64(v, $n),
                    _mm_srli_epi64(v, 48 - $n),
                );
                _mm_and_si128(r, _mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_rol_ $n _u $word>](v: __m128i) -> __m128i {
                _mm_or_si128([<_mm_slli_epi $word>](v, $n), [<_mm_srli_epi $word>](v, $word - $n))
            }
        }
    };
}

define_avx_rol!(16, 7);
define_avx_rol!(16, 2);
define_avx_rol!(24, 8);
define_avx_rol!(24, 3);
define_avx_rol!(32, 8);
define_avx_rol!(32, 3);
define_avx_rol!(48, 8);
define_avx_rol!(48, 3);
define_avx_rol!(64, 8);
define_avx_rol!(64, 3);

macro_rules! define_avx_add {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn avx_add_u24(a: __m128i, b: __m128i) -> __m128i {
                let s = _mm_add_epi32(a, b);
                _mm_and_si128(s, _mm_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn avx_add_u48(a: __m128i, b: __m128i) -> __m128i {
                let s = _mm_add_epi64(a, b);
                _mm_and_si128(s, _mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_add_u $word>](a: __m128i, b: __m128i) -> __m128i {
                [<_mm_add_epi $word>](a, b)
            }
        }
    };
}

define_avx_add!(16);
define_avx_add!(24);
define_avx_add!(32);
define_avx_add!(48);
define_avx_add!(64);

macro_rules! define_avx_sub {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn avx_sub_u24(a: __m128i, b: __m128i) -> __m128i {
                let s = _mm_sub_epi32(a, b);
                _mm_and_si128(s, _mm_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn avx_sub_u48(a: __m128i, b: __m128i) -> __m128i {
                let s = _mm_sub_epi64(a, b);
                _mm_and_si128(s, _mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_sub_u $word>](a: __m128i, b: __m128i) -> __m128i {
                [<_mm_sub_epi $word>](a, b)
            }
        }
    };
}

define_avx_sub!(16);
define_avx_sub!(24);
define_avx_sub!(32);
define_avx_sub!(48);
define_avx_sub!(64);

macro_rules! define_avx_xor {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn avx_xor_u24(a: __m128i, b: __m128i) -> __m128i {
                _mm_and_si128(_mm_xor_si128(a, b), _mm_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn avx_xor_u48(a: __m128i, b: __m128i) -> __m128i {
                _mm_and_si128(_mm_xor_si128(a, b), _mm_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_xor_u $word>](a: __m128i, b: __m128i) -> __m128i {
                _mm_xor_si128(a, b)
            }
        }
    };
}

define_avx_xor!(16);
define_avx_xor!(24);
define_avx_xor!(32);
define_avx_xor!(48);
define_avx_xor!(64);
