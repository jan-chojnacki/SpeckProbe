use crate::backend::avx::encrypt_round::*;
use paste::paste;
use std::arch::x86_64::*;

macro_rules! l_words {
    ($key:expr, 2) => {
        [$key[0]]
    };
    ($key:expr, 3) => {
        [$key[1], $key[0]]
    };
    ($key:expr, 4) => {
        [$key[2], $key[1], $key[0]]
    };
}

macro_rules! word_ty {
    (16) => {
        i16
    };
    (24) => {
        i32
    };
    (32) => {
        i32
    };
    (48) => {
        i64
    };
    (64) => {
        i64
    };
}

macro_rules! impl_expand_key_avx {
    ($block:literal, $key:literal, $word:literal, $avx_word: literal, $key_words:literal, $rounds:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
            #[target_feature(enable = "avx")]
            pub fn [<avx_expand_key_ $block _ $key>](key: [__m128i; $key_words]) -> [__m128i; $rounds ] {
                let mut rk: [__m128i; $rounds] = [_mm_setzero_si128(); $rounds];
                let mut l = l_words!(key, $key_words);
                let mut k = key[$key_words - 1];

                let mut i = 0usize;
                while i < $rounds {
                    rk[i] = k;
                    #[allow(clippy::modulo_one)]
                    let idx = i % ($key_words - 1);
                    [<avx_encrypt_round_ $word>](&mut l[idx], &mut k, [<_mm_set1_epi $avx_word>](i as word_ty!($word)));
                    i += 1;
                }
                rk
            }
        }
    };
}

impl_expand_key_avx!(32, 64, 16, 16, 4, 22);
impl_expand_key_avx!(48, 72, 24, 32, 3, 22);
impl_expand_key_avx!(48, 96, 24, 32, 4, 23);
impl_expand_key_avx!(64, 96, 32, 32, 3, 26);
impl_expand_key_avx!(64, 128, 32, 32, 4, 27);
impl_expand_key_avx!(96, 96, 48, 64x, 2, 28);
impl_expand_key_avx!(96, 144, 48, 64x, 3, 29);
impl_expand_key_avx!(128, 128, 64, 64x, 2, 32);
impl_expand_key_avx!(128, 192, 64, 64x, 3, 33);
impl_expand_key_avx!(128, 256, 64, 64x, 4, 34);
