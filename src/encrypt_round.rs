use crate::constants::*;
use crate::operations::*;
use paste::paste;

macro_rules! word_ty {
    (16) => {
        u16
    };
    (24) => {
        u32
    };
    (32) => {
        u32
    };
    (48) => {
        u64
    };
    (64) => {
        u64
    };
}

macro_rules! define_encrypt_round {
    ($w:literal) => {
        paste! {
            #[inline(always)]
            fn [<encrypt_round_ $w>](
                x: &mut word_ty!($w),
                y: &mut word_ty!($w),
                k: word_ty!($w),
            ) {
                let xr = [<ror_u $w>](*x, [<ALPHA_ $w>]);
                let s  = [<add_u $w>](xr, *y);
                *x     = [<xor_u $w>](s, k);

                let yl = [<rol_u $w>](*y, [<BETA_ $w>]);
                *y     = [<xor_u $w>](yl, *x);
            }
        }
    };
}

define_encrypt_round!(16);
define_encrypt_round!(24);
define_encrypt_round!(32);
define_encrypt_round!(48);
define_encrypt_round!(64);
