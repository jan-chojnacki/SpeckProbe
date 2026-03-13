use crate::backend::avx512::encrypt_round::*;
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

macro_rules! impl_expand_key_avx512 {
    ($block:literal, $key:literal, $word:literal, $avx_word: literal, $key_words:literal, $rounds:literal, $feature:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = $feature))]
            #[target_feature(enable = $feature)]
            pub fn [<avx512_expand_key_ $block _ $key>](key: [__m512i; $key_words]) -> [__m512i; $rounds ] {
                let mut rk: [__m512i; $rounds] = [_mm512_setzero_si512(); $rounds];
                let mut l = l_words!(key, $key_words);
                let mut k = key[$key_words - 1];

                let mut i = 0usize;
                while i < $rounds {
                    rk[i] = k;
                    #[allow(clippy::modulo_one)]
                    let idx = i % ($key_words - 1);
                    [<avx512_encrypt_round_ $word>](&mut l[idx], &mut k, [<_mm512_set1_epi $avx_word>](i as word_ty!($word)));
                    i += 1;
                }
                rk
            }
        }
    };
}

impl_expand_key_avx512!(32, 64, 16, 16, 4, 22, "avx512bw");
impl_expand_key_avx512!(48, 72, 24, 32, 3, 22, "avx512f");
impl_expand_key_avx512!(48, 96, 24, 32, 4, 23, "avx512f");
impl_expand_key_avx512!(64, 96, 32, 32, 3, 26, "avx512f");
impl_expand_key_avx512!(64, 128, 32, 32, 4, 27, "avx512f");
impl_expand_key_avx512!(96, 96, 48, 64, 2, 28, "avx512f");
impl_expand_key_avx512!(96, 144, 48, 64, 3, 29, "avx512f");
impl_expand_key_avx512!(128, 128, 64, 64, 2, 32, "avx512f");
impl_expand_key_avx512!(128, 192, 64, 64, 3, 33, "avx512f");
impl_expand_key_avx512!(128, 256, 64, 64, 4, 34, "avx512f");
