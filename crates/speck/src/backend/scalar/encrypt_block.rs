use crate::backend::scalar::encrypt_round::*;
use crate::backend::scalar::expand_key::*;
use crate::backend::scalar::word_ty;
use paste::paste;

macro_rules! impl_encrypt_block {
    ($block:literal, $key:literal, $word:literal, $key_words:literal) => {
        paste! {
            #[inline(always)]
            pub fn [<encrypt_block_ $block _ $key>](ct: [word_ty!($word); 2], key: [word_ty!($word); $key_words]) -> [word_ty!($word); 2] {
                let round_keys = [<expand_key_ $block _ $key>](key);

                let mut x = ct[0];
                let mut y = ct[1];

                for k in round_keys {
                    [<encrypt_round_ $word>](&mut x, &mut y, k);
                }

                [x, y]
            }
        }
    };
}

impl_encrypt_block!(32, 64, 16, 4);
impl_encrypt_block!(48, 72, 24, 3);
impl_encrypt_block!(48, 96, 24, 4);
impl_encrypt_block!(64, 96, 32, 3);
impl_encrypt_block!(64, 128, 32, 4);
impl_encrypt_block!(96, 96, 48, 2);
impl_encrypt_block!(96, 144, 48, 3);
impl_encrypt_block!(128, 128, 64, 2);
impl_encrypt_block!(128, 192, 64, 3);
impl_encrypt_block!(128, 256, 64, 4);
