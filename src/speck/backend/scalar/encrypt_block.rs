use super::encrypt_round_inline;
use super::expand_key_inline;
use crate::speck::backend::{U24, U48};
use crate::speck::impl_adapter;
use crate::speck::key_idx;
use crate::speck::key_words_inline;
use seq_macro::seq;
use std::ops::BitXor;

macro_rules! impl_encrypt_block {
    ($fn_name:ident, $word:ty, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[inline(always)]
        pub fn $fn_name(ct: [$word; 2], key: [$word; $key_words]) -> [$word; 2] {
            let mut round_keys: [$word; $rounds + 1] = [Default::default(); $rounds + 1];
            expand_key_inline!(round_keys, key, $word, $key_words, $alpha, $beta, $rounds);

            let mut x = ct[0];
            let mut y = ct[1];

            seq!(I in 0..=$rounds {
                encrypt_round_inline!(x, y, round_keys[I], $alpha, $beta);
            });

            [x, y]
        }
    };
}

impl_encrypt_block!(scalar_encrypt_block_32_64, u16, 4, 7, 2, 21);
impl_encrypt_block!(scalar_encrypt_block_48_72_u24, U24, 3, 8, 3, 21);
impl_encrypt_block!(scalar_encrypt_block_48_96_u24, U24, 4, 8, 3, 22);
impl_encrypt_block!(scalar_encrypt_block_64_96, u32, 3, 8, 3, 25);
impl_encrypt_block!(scalar_encrypt_block_64_128, u32, 4, 8, 3, 26);
impl_encrypt_block!(scalar_encrypt_block_96_96_u48, U48, 2, 8, 3, 27);
impl_encrypt_block!(scalar_encrypt_block_96_144_u48, U48, 3, 8, 3, 28);
impl_encrypt_block!(scalar_encrypt_block_128_128, u64, 2, 8, 3, 31);
impl_encrypt_block!(scalar_encrypt_block_128_192, u64, 3, 8, 3, 32);
impl_encrypt_block!(scalar_encrypt_block_128_256, u64, 4, 8, 3, 33);

impl_adapter!(
    scalar_encrypt_block_48_72,
    scalar_encrypt_block_48_72_u24,
    U24,
    u32,
    3
);
impl_adapter!(
    scalar_encrypt_block_48_96,
    scalar_encrypt_block_48_96_u24,
    U24,
    u32,
    4
);
impl_adapter!(
    scalar_encrypt_block_96_96,
    scalar_encrypt_block_96_96_u48,
    U48,
    u64,
    2
);
impl_adapter!(
    scalar_encrypt_block_96_144,
    scalar_encrypt_block_96_144_u48,
    U48,
    u64,
    3
);

macro_rules! impl_encrypt_block_inflight {
    ($fn_name:ident, $word:ty, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[inline(always)]
        pub fn $fn_name(ct: [$word; 2], key: [$word; $key_words]) -> [$word; 2] {
            let mut l = key_words_inline!(key, $key_words);
            let mut k = key[$key_words - 1];

            let mut x = ct[0];
            let mut y = ct[1];

            seq!(I in 0..$rounds {
                encrypt_round_inline!(x, y, k, $alpha, $beta);
                encrypt_round_inline!(l[key_idx!($key_words, I)],
                    k, <$word as From<u8>>::from(I as u8), $alpha, $beta);
            });
            encrypt_round_inline!(x, y, k, $alpha, $beta);

            [x, y]
        }
    };
}

impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_32_64, u16, 4, 7, 2, 21);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_48_72_u24, U24, 3, 8, 3, 21);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_48_96_u24, U24, 4, 8, 3, 22);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_64_96, u32, 3, 8, 3, 25);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_64_128, u32, 4, 8, 3, 26);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_96_96_u48, U48, 2, 8, 3, 27);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_96_144_u48, U48, 3, 8, 3, 28);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_128_128, u64, 2, 8, 3, 31);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_128_192, u64, 3, 8, 3, 32);
impl_encrypt_block_inflight!(scalar_encrypt_block_inflight_128_256, u64, 4, 8, 3, 33);

impl_adapter!(
    scalar_encrypt_block_inflight_48_72,
    scalar_encrypt_block_inflight_48_72_u24,
    U24,
    u32,
    3
);
impl_adapter!(
    scalar_encrypt_block_inflight_48_96,
    scalar_encrypt_block_inflight_48_96_u24,
    U24,
    u32,
    4
);
impl_adapter!(
    scalar_encrypt_block_inflight_96_96,
    scalar_encrypt_block_inflight_96_96_u48,
    U48,
    u64,
    2
);
impl_adapter!(
    scalar_encrypt_block_inflight_96_144,
    scalar_encrypt_block_inflight_96_144_u48,
    U48,
    u64,
    3
);
