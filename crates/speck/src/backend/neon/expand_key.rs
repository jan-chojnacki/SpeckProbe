use crate::backend::neon::encrypt_round::*;
#[cfg(target_arch = "aarch64")]
use crate::backend::neon::neon_word_ty;
use paste::paste;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t, vdupq_n_u16, vdupq_n_u32, vdupq_n_u64};


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
        u16
    };
    (24) => {
        u32
    };
    (32) => {
        u32
    };
    (48) => {
        u64
    };
    (64) => {
        u64
    };
}

macro_rules! impl_expand_key_neon {
    ($block:literal, $key:literal, $word:literal, $neon_word: literal, $key_words:literal, $rounds:literal) => {
        paste! {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_expand_key_ $block _ $key>](key: [neon_word_ty!($word); $key_words]) -> [neon_word_ty!($word); $rounds ] {
                let mut rk: [neon_word_ty!($word); $rounds] = [[<vdupq_n_u $neon_word>](0); $rounds];
                let mut l = l_words!(key, $key_words);
                let mut k = key[$key_words - 1];

                let mut i = 0usize;
                while i < $rounds {
                    rk[i] = k;
                    #[allow(clippy::modulo_one)]
                    let idx = i % ($key_words - 1);
                    [<neon_encrypt_round_ $word>](&mut l[idx], &mut k, [<vdupq_n_u $neon_word>](i as word_ty!($word)));
                    i += 1;
                }
                rk
            }
        }
    };
}

impl_expand_key_neon!(32, 64, 16, 16, 4, 22);
impl_expand_key_neon!(48, 72, 24, 32, 3, 22);
impl_expand_key_neon!(48, 96, 24, 32, 4, 23);
impl_expand_key_neon!(64, 96, 32, 32, 3, 26);
impl_expand_key_neon!(64, 128, 32, 32, 4, 27);
impl_expand_key_neon!(96, 96, 48, 64, 2, 28);
impl_expand_key_neon!(96, 144, 48, 64, 3, 29);
impl_expand_key_neon!(128, 128, 64, 64, 2, 32);
impl_expand_key_neon!(128, 192, 64, 64, 3, 33);
impl_expand_key_neon!(128, 256, 64, 64, 4, 34);
