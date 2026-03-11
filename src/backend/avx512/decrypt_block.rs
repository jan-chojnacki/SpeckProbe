use crate::backend::avx512::decrypt_round::*;
use crate::backend::avx512::expand_key::*;
use paste::paste;
use std::arch::x86_64::__m512i;

macro_rules! impl_decrypt_block_avx512 {
    ($block:literal, $key:literal, $word:literal, $key_words:literal, $feature:literal) => {
        paste! {
            #[cfg(all(target_arch = "x86_64", target_feature = $feature))]
            #[target_feature(enable = $feature)]
            pub fn [<avx512_decrypt_block_ $block _ $key>](ct: [__m512i; 2], key: [__m512i; $key_words]) -> [__m512i; 2] {
                let round_keys = [<avx512_expand_key_ $block _ $key>](key);

                let mut x = ct[0];
                let mut y = ct[1];

                for &k in round_keys.iter().rev() {
                    [<avx512_decrypt_round_ $word>](&mut x, &mut y, k);
                }

                [x, y]
            }
        }
    };
}

impl_decrypt_block_avx512!(32, 64, 16, 4, "avx512bw");
impl_decrypt_block_avx512!(48, 72, 24, 3, "avx512f");
impl_decrypt_block_avx512!(48, 96, 24, 4, "avx512f");
impl_decrypt_block_avx512!(64, 96, 32, 3, "avx512f");
impl_decrypt_block_avx512!(64, 128, 32, 4, "avx512f");
impl_decrypt_block_avx512!(96, 96, 48, 2, "avx512f");
impl_decrypt_block_avx512!(96, 144, 48, 3, "avx512f");
impl_decrypt_block_avx512!(128, 128, 64, 2, "avx512f");
impl_decrypt_block_avx512!(128, 192, 64, 3, "avx512f");
impl_decrypt_block_avx512!(128, 256, 64, 4, "avx512f");
