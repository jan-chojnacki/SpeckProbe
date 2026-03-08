use paste::paste;

macro_rules! define_speck_consts_rounds {
    ($block:literal, $key:literal, $rounds:expr $(,)?) => {
        paste! {
            pub const [<ROUNDS_ $block _ $key>]: usize = $rounds;
        }
    };
}

macro_rules! define_speck_consts_alpha_beta {
    ($word:literal, $alpha:expr, $beta:expr $(,)?) => {
        paste! {
            pub const [<ALPHA_ $word>]: u32 = $alpha;
            pub const [<BETA_ $word>]: u32 = $beta;
        }
    };
}

define_speck_consts_rounds!(32, 64, 22);
define_speck_consts_rounds!(48, 72, 22);
define_speck_consts_rounds!(48, 96, 23);
define_speck_consts_rounds!(64, 96, 26);
define_speck_consts_rounds!(64, 128, 27);
define_speck_consts_rounds!(96, 96, 28);
define_speck_consts_rounds!(96, 144, 29);
define_speck_consts_rounds!(128, 128, 32);
define_speck_consts_rounds!(128, 192, 33);
define_speck_consts_rounds!(128, 256, 34);

define_speck_consts_alpha_beta!(16, 7, 2);
define_speck_consts_alpha_beta!(24, 8, 3);
define_speck_consts_alpha_beta!(32, 8, 3);
define_speck_consts_alpha_beta!(48, 8, 3);
define_speck_consts_alpha_beta!(64, 8, 3);