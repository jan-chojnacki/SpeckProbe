use crate::__m256i;
use paste::paste;
use std::arch::x86_64::_mm256_add_epi16;
use std::arch::x86_64::_mm256_add_epi32;
use std::arch::x86_64::_mm256_add_epi64;
use std::arch::x86_64::_mm256_and_si256;
use std::arch::x86_64::_mm256_or_si256;
use std::arch::x86_64::_mm256_set1_epi32;
use std::arch::x86_64::_mm256_set1_epi64x;
use std::arch::x86_64::_mm256_slli_epi16;
use std::arch::x86_64::_mm256_slli_epi32;
use std::arch::x86_64::_mm256_slli_epi64;
use std::arch::x86_64::_mm256_srli_epi16;
use std::arch::x86_64::_mm256_srli_epi32;
use std::arch::x86_64::_mm256_srli_epi64;
use std::arch::x86_64::_mm256_sub_epi16;
use std::arch::x86_64::_mm256_sub_epi32;
use std::arch::x86_64::_mm256_sub_epi64;
use std::arch::x86_64::_mm256_xor_si256;

macro_rules! define_avx2_ror {
    (24, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_ror_ $n _u24>](v: __m256i) -> __m256i {
                let r = _mm256_or_si256(
                    _mm256_srli_epi32(v, $n),
                    _mm256_slli_epi32(v, 24 - $n),
                );
                _mm256_and_si256(r, _mm256_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_ror_ $n _u48>](v: __m256i) -> __m256i {
                let r = _mm256_or_si256(
                    _mm256_srli_epi64(v, $n),
                    _mm256_slli_epi64(v, 48 - $n),
                );
                _mm256_and_si256(r, _mm256_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_ror_ $n _u $word>](v: __m256i) -> __m256i {
                _mm256_or_si256([<_mm256_srli_epi $word>](v, $n), [<_mm256_slli_epi $word>](v, $word - $n))
            }
        }
    };
}

define_avx2_ror!(16, 8);
define_avx2_ror!(16, 7);
define_avx2_ror!(16, 3);
define_avx2_ror!(16, 2);
define_avx2_ror!(24, 8);
define_avx2_ror!(24, 7);
define_avx2_ror!(24, 3);
define_avx2_ror!(24, 2);
define_avx2_ror!(32, 8);
define_avx2_ror!(32, 7);
define_avx2_ror!(32, 3);
define_avx2_ror!(32, 2);
define_avx2_ror!(48, 8);
define_avx2_ror!(48, 7);
define_avx2_ror!(48, 3);
define_avx2_ror!(48, 2);
define_avx2_ror!(64, 8);
define_avx2_ror!(64, 7);
define_avx2_ror!(64, 3);
define_avx2_ror!(64, 2);

macro_rules! define_avx2_rol {
    (24, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_rol_ $n _u24>](v: __m256i) -> __m256i {
                let r = _mm256_or_si256(
                    _mm256_slli_epi32(v, $n),
                    _mm256_srli_epi32(v, 24 - $n),
                );
                _mm256_and_si256(r, _mm256_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_rol_ $n _u48>](v: __m256i) -> __m256i {
                let r = _mm256_or_si256(
                    _mm256_slli_epi64(v, $n),
                    _mm256_srli_epi64(v, 48 - $n),
                );
                _mm256_and_si256(r, _mm256_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal, $n:expr) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_rol_ $n _u $word>](v: __m256i) -> __m256i {
                _mm256_or_si256([<_mm256_slli_epi $word>](v, $n), [<_mm256_srli_epi $word>](v, $word - $n))
            }
        }
    };
}

define_avx2_rol!(16, 8);
define_avx2_rol!(16, 7);
define_avx2_rol!(16, 3);
define_avx2_rol!(16, 2);
define_avx2_rol!(24, 8);
define_avx2_rol!(24, 7);
define_avx2_rol!(24, 3);
define_avx2_rol!(24, 2);
define_avx2_rol!(32, 8);
define_avx2_rol!(32, 7);
define_avx2_rol!(32, 3);
define_avx2_rol!(32, 2);
define_avx2_rol!(48, 8);
define_avx2_rol!(48, 7);
define_avx2_rol!(48, 3);
define_avx2_rol!(48, 2);
define_avx2_rol!(64, 8);
define_avx2_rol!(64, 7);
define_avx2_rol!(64, 3);
define_avx2_rol!(64, 2);

macro_rules! define_avx2_add {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn avx2_add_u24(a: __m256i, b: __m256i) -> __m256i {
                let s = _mm256_add_epi32(a, b);
                _mm256_and_si256(s, _mm256_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn avx2_add_u48(a: __m256i, b: __m256i) -> __m256i {
                let s = _mm256_add_epi64(a, b);
                _mm256_and_si256(s, _mm256_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_add_u $word>](a: __m256i, b: __m256i) -> __m256i {
                [<_mm256_add_epi $word>](a, b)
            }
        }
    };
}

define_avx2_add!(16);
define_avx2_add!(24);
define_avx2_add!(32);
define_avx2_add!(48);
define_avx2_add!(64);

macro_rules! define_avx2_sub {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn avx2_sub_u24(a: __m256i, b: __m256i) -> __m256i {
                let s = _mm256_sub_epi32(a, b);
                _mm256_and_si256(s, _mm256_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn avx2_sub_u48(a: __m256i, b: __m256i) -> __m256i {
                let s = _mm256_sub_epi64(a, b);
                _mm256_and_si256(s, _mm256_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_sub_u $word>](a: __m256i, b: __m256i) -> __m256i {
                [<_mm256_sub_epi $word>](a, b)
            }
        }
    };
}

define_avx2_sub!(16);
define_avx2_sub!(24);
define_avx2_sub!(32);
define_avx2_sub!(48);
define_avx2_sub!(64);

macro_rules! define_avx2_xor {
    (24) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn avx2_xor_u24(a: __m256i, b: __m256i) -> __m256i {
                _mm256_and_si256(_mm256_xor_si256(a, b), _mm256_set1_epi32(0x00FF_FFFFu32 as i32))
            }
        }
    };

    (48) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn avx2_xor_u48(a: __m256i, b: __m256i) -> __m256i {
                _mm256_and_si256(_mm256_xor_si256(a, b), _mm256_set1_epi64x(0x0000_FFFF_FFFF_FFFFu64 as i64))
            }
        }
    };

    ($word:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            #[target_feature(enable = "avx2")]
            pub fn [<avx2_xor_u $word>](a: __m256i, b: __m256i) -> __m256i {
                _mm256_xor_si256(a, b)
            }
        }
    };
}

define_avx2_xor!(16);
define_avx2_xor!(24);
define_avx2_xor!(32);
define_avx2_xor!(48);
define_avx2_xor!(64);
