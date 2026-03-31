use crate::backend::common::{U24, U48};
use crate::encrypt_round_inline;
use crate::expand_key_inline;
use crate::key_idx;
use crate::key_words_inline;
use crate::{decrypt_round_inline, impl_adapter};
use seq_macro::seq;
use std::ops::BitXor;

macro_rules! impl_decrypt_block {
    ($fn_name:ident, $word:ty, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[inline(always)]
        pub fn $fn_name(ct: [$word; 2], key: [$word; $key_words]) -> [$word; 2] {
            let mut round_keys: [$word; $rounds + 1] = [Default::default(); $rounds + 1];
            expand_key_inline!(round_keys, key, $word, $key_words, $alpha, $beta, $rounds);

            let mut x = ct[0];
            let mut y = ct[1];

            seq!(I in 0..=$rounds {
                decrypt_round_inline!(x, y, round_keys[$rounds - I], $alpha, $beta);
            });

            [x, y]
        }
    };
}

impl_decrypt_block!(decrypt_block_32_64, u16, 4, 7, 2, 21);
impl_decrypt_block!(decrypt_block_48_72_u24, U24, 3, 8, 3, 21);
impl_decrypt_block!(decrypt_block_48_96_u24, U24, 4, 8, 3, 22);
impl_decrypt_block!(decrypt_block_64_96, u32, 3, 8, 3, 25);
impl_decrypt_block!(decrypt_block_64_128, u32, 4, 8, 3, 26);
impl_decrypt_block!(decrypt_block_96_96_u48, U48, 2, 8, 3, 27);
impl_decrypt_block!(decrypt_block_96_144_48, U48, 3, 8, 3, 28);
impl_decrypt_block!(decrypt_block_128_128, u64, 2, 8, 3, 31);
impl_decrypt_block!(decrypt_block_128_192, u64, 3, 8, 3, 32);
impl_decrypt_block!(decrypt_block_128_256, u64, 4, 8, 3, 33);

impl_adapter!(decrypt_block_48_72, decrypt_block_48_72_u24, U24, u32, 3);
impl_adapter!(decrypt_block_48_96, decrypt_block_48_96_u24, U24, u32, 4);
impl_adapter!(decrypt_block_96_96, decrypt_block_96_96_u48, U48, u64, 2);
impl_adapter!(decrypt_block_96_144, decrypt_block_96_144_48, U48, u64, 3);
