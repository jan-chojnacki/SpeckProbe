macro_rules! expand_key_inline {
    ($round_keys:expr, $key:expr, $word:ty, $key_words:tt, $alpha:literal, $beta:literal, $rounds:expr) => {
        let mut l = key_words_inline!($key, $key_words);
        let mut k = $key[$key_words - 1];


        seq!(I in 0..$rounds {
            $round_keys[I] = k;
            encrypt_round_inline!(l[key_idx!($key_words, I)],
                k, <$word as From<u8>>::from(I as u8), $alpha, $beta);
        });

        $round_keys[$rounds] = k;
    };
}

pub(crate) use expand_key_inline;
