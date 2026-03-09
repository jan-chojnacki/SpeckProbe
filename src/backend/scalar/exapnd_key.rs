use crate::backend::scalar::encrypt_round::*;
use crate::backend::scalar::word_ty;
use paste::paste;

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

macro_rules! impl_expand_key {
    ($block:literal, $key:literal, $word:literal, $key_words:literal, $rounds:literal) => {
        paste! {
            pub fn [<expand_key_ $block _ $key>](key: [word_ty!($word); $key_words]) -> [word_ty!($word); $rounds ] {
                let mut rk: [word_ty!($word); $rounds] = [0; $rounds];
                let mut l = l_words!(key, $key_words);
                let mut k = key[$key_words - 1];

                let mut i = 0usize;
                while i < $rounds {
                    rk[i] = k;
                    let idx = i % ($key_words - 1);
                    [<encrypt_round_ $word>](&mut l[idx], &mut k, i as word_ty!($word));
                    i += 1;
                }
                rk
            }
        }
    };
}

impl_expand_key!(32, 64, 16, 4, 22);
impl_expand_key!(48, 72, 24, 3, 22);
impl_expand_key!(48, 96, 24, 4, 23);
impl_expand_key!(64, 96, 32, 3, 26);
impl_expand_key!(64, 128, 32, 4, 27);
impl_expand_key!(96, 96, 48, 2, 28);
impl_expand_key!(96, 144, 48, 3, 29);
impl_expand_key!(128, 128, 64, 2, 32);
impl_expand_key!(128, 192, 64, 3, 33);
impl_expand_key!(128, 256, 64, 4, 34);
