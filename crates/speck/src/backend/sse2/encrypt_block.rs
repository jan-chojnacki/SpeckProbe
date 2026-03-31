use crate::backend::sse2::encrypt_round::*;
use crate::backend::sse2::expand_key::*;
use crate::key_idx;
use crate::key_words_inline;
use crate::sse2_add;
use crate::sse2_rol;
use crate::sse2_ror;
use crate::sse2_set;
use crate::sse2_sub;
use crate::sse2_xor;
use crate::{U24, U48, sse2_encrypt_round_inline};
use paste::paste;
use seq_macro::seq;
use std::arch::x86_64::__m128i;

macro_rules! impl_encrypt_block_sse2 {
    ($block:literal, $key:literal, $word:literal, $key_words:literal) => {
        paste! {
            #[doc = concat!(
                "Encrypts one Speck block (",
                stringify!($block),
                "/",
                stringify!($key),
                ") using AVX."
            )]
            #[doc = ""]
            #[doc = "# Safety"]
            #[doc = concat!(
                "Caller must ensure CPU support for `sse2` before calling this function."
            )]
            #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
            #[target_feature(enable = "sse2")]
            pub fn [<sse2_encrypt_block_ $block _ $key>](ct: [__m128i; 2], key: [__m128i; $key_words]) -> [__m128i; 2] {
                let round_keys = [<sse2_expand_key_ $block _ $key>](key);

                let mut x = ct[0];
                let mut y = ct[1];

                for k in round_keys {
                    [<sse2_encrypt_round_ $word>](&mut x, &mut y, k);
                }

                [x, y]
            }
        }
    };
}

macro_rules! impl_encrypt_block_sse2_inflight {
    ($fn_name:ident, $word:tt, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        #[doc = concat!(
            "Encrypts one Speck block using AVX."
        )]
        #[doc = ""]
        #[doc = "# Safety"]
        #[doc = concat!(
            "Caller must ensure CPU support for `sse2` before calling this function."
        )]
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        #[target_feature(enable = "sse2")]
        pub fn $fn_name(ct: [__m128i; 2], key: [__m128i; $key_words]) -> [__m128i; 2] {
            let mut l = key_words_inline!(key, $key_words);
            let mut k = key[$key_words - 1];

            let mut x = ct[0];
            let mut y = ct[1];

            seq!(I in 0..$rounds {
                sse2_encrypt_round_inline!(x, y, k, $word, $alpha, $beta);
                sse2_encrypt_round_inline!(l[key_idx!($key_words, I)],
                    k, sse2_set!($word, I), $word, $alpha, $beta);
            });
            sse2_encrypt_round_inline!(x, y, k, $word, $alpha, $beta);

            [x, y]
        }
    };
}

impl_encrypt_block_sse2!(32, 64, 16, 4);
impl_encrypt_block_sse2!(48, 72, 24, 3);
impl_encrypt_block_sse2!(48, 96, 24, 4);
impl_encrypt_block_sse2!(64, 96, 32, 3);
impl_encrypt_block_sse2!(64, 128, 32, 4);
impl_encrypt_block_sse2!(96, 96, 48, 2);
impl_encrypt_block_sse2!(96, 144, 48, 3);
impl_encrypt_block_sse2!(128, 128, 64, 2);
impl_encrypt_block_sse2!(128, 192, 64, 3);
impl_encrypt_block_sse2!(128, 256, 64, 4);

impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_32_64, 16, 4, 7, 2, 21);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_48_72, 24, 3, 8, 3, 21);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_48_96, 24, 4, 8, 3, 22);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_64_96, 32, 3, 8, 3, 25);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_64_128, 32, 4, 8, 3, 26);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_96_96, 48, 2, 8, 3, 27);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_96_144, 48, 3, 8, 3, 28);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_128_128, 64, 2, 8, 3, 31);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_128_192, 64, 3, 8, 3, 32);
impl_encrypt_block_sse2_inflight!(sse2_encrypt_block_inflight_128_256, 64, 4, 8, 3, 33);
