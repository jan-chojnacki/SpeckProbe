use crate::backend::scalar::decrypt_round::*;
use crate::backend::scalar::expand_key::*;
use paste::paste;
use seq_macro::seq;
use std::ops::BitXor;

use crate::backend::scalar::word_ty;
use crate::{U24, U48};

macro_rules! round_idx {
    (2, $i:literal) => {
        0
    };

    (3,  0) => {
        0
    };
    (3,  1) => {
        1
    };
    (3,  2) => {
        0
    };
    (3,  3) => {
        1
    };
    (3,  4) => {
        0
    };
    (3,  5) => {
        1
    };
    (3,  6) => {
        0
    };
    (3,  7) => {
        1
    };
    (3,  8) => {
        0
    };
    (3,  9) => {
        1
    };
    (3, 10) => {
        0
    };
    (3, 11) => {
        1
    };
    (3, 12) => {
        0
    };
    (3, 13) => {
        1
    };
    (3, 14) => {
        0
    };
    (3, 15) => {
        1
    };
    (3, 16) => {
        0
    };
    (3, 17) => {
        1
    };
    (3, 18) => {
        0
    };
    (3, 19) => {
        1
    };
    (3, 20) => {
        0
    };
    (3, 21) => {
        1
    };
    (3, 22) => {
        0
    };
    (3, 23) => {
        1
    };
    (3, 24) => {
        0
    };
    (3, 25) => {
        1
    };
    (3, 26) => {
        0
    };
    (3, 27) => {
        1
    };
    (3, 28) => {
        0
    };
    (3, 29) => {
        1
    };
    (3, 30) => {
        0
    };
    (3, 31) => {
        1
    };
    (3, 32) => {
        0
    };
    (3, 33) => {
        1
    };

    (4,  0) => {
        0
    };
    (4,  1) => {
        1
    };
    (4,  2) => {
        2
    };
    (4,  3) => {
        0
    };
    (4,  4) => {
        1
    };
    (4,  5) => {
        2
    };
    (4,  6) => {
        0
    };
    (4,  7) => {
        1
    };
    (4,  8) => {
        2
    };
    (4,  9) => {
        0
    };
    (4, 10) => {
        1
    };
    (4, 11) => {
        2
    };
    (4, 12) => {
        0
    };
    (4, 13) => {
        1
    };
    (4, 14) => {
        2
    };
    (4, 15) => {
        0
    };
    (4, 16) => {
        1
    };
    (4, 17) => {
        2
    };
    (4, 18) => {
        0
    };
    (4, 19) => {
        1
    };
    (4, 20) => {
        2
    };
    (4, 21) => {
        0
    };
    (4, 22) => {
        1
    };
    (4, 23) => {
        2
    };
    (4, 24) => {
        0
    };
    (4, 25) => {
        1
    };
    (4, 26) => {
        2
    };
    (4, 27) => {
        0
    };
    (4, 28) => {
        1
    };
    (4, 29) => {
        2
    };
    (4, 30) => {
        0
    };
    (4, 31) => {
        1
    };
    (4, 32) => {
        2
    };
    (4, 33) => {
        0
    };

    ($kw:literal, $i:literal) => {
        compile_error!("unsupported key_words / round index in round_idx!");
    };
}

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

macro_rules! define_encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $alpha:literal, $beta:literal) => {
        $x = $x.rotate_right($alpha).wrapping_add($y).bitxor($k);
        $y = $y.rotate_left($beta).bitxor($x);
    };
}

macro_rules! impl_expand_key {
    ($round_keys:expr, $key:expr, $word:ty, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        let mut l = l_words!($key, $key_words);
        let mut k = $key[$key_words - 1];


        seq!(I in 0..$rounds {
            $round_keys[I] = k;
            define_encrypt_round_inline!(l[round_idx!($key_words, I)], k, <$word as From<u8>>::from(I as u8), $alpha, $beta);
        });

        $round_keys[$rounds] = k;
    };
}

macro_rules! define_decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $alpha:literal, $beta:literal) => {
        $y = $y.bitxor($x).rotate_right($beta);
        $x = $x.bitxor($k).wrapping_sub($y).rotate_left($alpha);
    };
}

macro_rules! impl_decrypt_block {
    ($block:literal, $key:literal, $word:ty, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        paste! {
            #[inline(always)]
            pub fn [<decrypt_block_ $block _ $key>](ct: [$word; 2], key: [$word; $key_words]) -> [$word; 2] {
                let mut round_keys: [$word; $rounds + 1] = [Default::default(); $rounds + 1];
                impl_expand_key!(round_keys, key, $word, $key_words, $alpha, $beta, $rounds);

                let mut x = ct[0];
                let mut y = ct[1];

                seq!(I in 0..=$rounds {
                    define_decrypt_round_inline!(x, y, round_keys[$rounds - I], $alpha, $beta);
                });

                [x, y]
            }
        }
    };
}

impl_decrypt_block!(32, 64, u16, 4, 7, 2, 21);
impl_decrypt_block!(48, 72, U24, 3, 8, 3, 21);
impl_decrypt_block!(48, 96, U24, 4, 8, 3, 22);
impl_decrypt_block!(64, 96, u32, 3, 8, 3, 25);
impl_decrypt_block!(64, 128, u32, 4, 8, 3, 26);
impl_decrypt_block!(96, 96, U48, 2, 8, 3, 27);
impl_decrypt_block!(96, 144, U48, 3, 8, 3, 28);
impl_decrypt_block!(128, 128, u64, 2, 8, 3, 31);
impl_decrypt_block!(128, 192, u64, 3, 8, 3, 32);
impl_decrypt_block!(128, 256, u64, 4, 8, 3, 33);
