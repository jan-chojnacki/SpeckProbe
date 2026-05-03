macro_rules! key_words_inline {
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

pub(crate) use key_words_inline;
