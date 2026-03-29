use crate::backend::common::{U24, U48};
use crate::decrypt_round_inline;
use crate::encrypt_round_inline;
use crate::expand_key_inline;
use crate::key_idx;
use crate::key_words_inline;
use paste::paste;
use seq_macro::seq;
use std::ops::BitXor;

macro_rules! impl_decrypt_block {
    ($block:literal, $key:literal, $word:ty, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        paste! {
            #[inline(always)]
            pub fn [<decrypt_block_ $block _ $key>](ct: [$word; 2], key: [$word; $key_words]) -> [$word; 2] {
                let mut round_keys: [$word; $rounds + 1] = [Default::default(); $rounds + 1];
                expand_key_inline!(round_keys, key, $word, $key_words, $alpha, $beta, $rounds);

                let mut x = ct[0];
                let mut y = ct[1];

                seq!(I in 0..=$rounds {
                    decrypt_round_inline!(x, y, round_keys[$rounds - I], $alpha, $beta);
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
