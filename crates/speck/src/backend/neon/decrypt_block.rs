use crate::backend::neon::decrypt_round::*;
use crate::backend::neon::expand_key::*;
use crate::backend::neon::neon_word_ty;
use paste::paste;
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};

macro_rules! impl_decrypt_block_neon {
    ($block:literal, $key:literal, $word:literal, $key_words:literal) => {
        paste! {
            #[doc = concat!(
                "Decrypts one Speck block (",
                stringify!($block),
                "/",
                stringify!($key),
                ") using NEON."
            )]
            #[doc = ""]
            #[doc = "# Safety"]
            #[doc = concat!(
                "Caller must ensure CPU support for `neon` before calling this function."
            )]
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            #[target_feature(enable = "neon")]
            pub fn [<neon_decrypt_block_ $block _ $key>](ct: [neon_word_ty!($word); 2], key: [neon_word_ty!($word); $key_words]) -> [neon_word_ty!($word); 2] {
                let round_keys = [<neon_expand_key_ $block _ $key>](key);

                let mut x = ct[0];
                let mut y = ct[1];

                for &k in round_keys.iter().rev() {
                    [<neon_decrypt_round_ $word>](&mut x, &mut y, k);
                }

                [x, y]
            }
        }
    };
}

impl_decrypt_block_neon!(32, 64, 16, 4);
impl_decrypt_block_neon!(48, 72, 24, 3);
impl_decrypt_block_neon!(48, 96, 24, 4);
impl_decrypt_block_neon!(64, 96, 32, 3);
impl_decrypt_block_neon!(64, 128, 32, 4);
impl_decrypt_block_neon!(96, 96, 48, 2);
impl_decrypt_block_neon!(96, 144, 48, 3);
impl_decrypt_block_neon!(128, 128, 64, 2);
impl_decrypt_block_neon!(128, 192, 64, 3);
impl_decrypt_block_neon!(128, 256, 64, 4);
