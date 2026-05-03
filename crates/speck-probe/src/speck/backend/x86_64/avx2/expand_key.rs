macro_rules! avx2_expand_key_inline {
    ($round_keys:expr, $key:expr, $word:tt, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        let mut l = $crate::speck::key_words_inline!($key, $key_words);
        let mut k = $key[$key_words - 1];

        seq_macro::seq!(I in 0..$rounds {
            $round_keys[I] = k;
            $crate::speck::backend::x86_64::avx2::avx2_encrypt_round_inline!(l[$crate::speck::key_idx!($key_words, I)],
                    k, $crate::speck::backend::x86_64::avx2::avx2_set!($word, I), $word, $alpha, $beta);
        });

        $round_keys[$rounds] = k;
    };
}

pub(crate) use avx2_expand_key_inline;
