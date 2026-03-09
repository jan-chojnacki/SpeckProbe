use crate::backend::scalar::operaions::*;
use crate::backend::scalar::word_ty;
use paste::paste;

macro_rules! define_encrypt_round {
    ($w:literal, $alpha:literal, $beta:literal) => {
        paste! {
            #[inline(always)]
            pub fn [<encrypt_round_ $w>](
                x: &mut word_ty!($w),
                y: &mut word_ty!($w),
                k: word_ty!($w),
            ) {
                let xr = [<ror_u $w>](*x, $alpha);
                let s  = [<add_u $w>](xr, *y);
                *x     = [<xor_u $w>](s, k);

                let yl = [<rol_u $w>](*y, $beta);
                *y     = [<xor_u $w>](yl, *x);
            }
        }
    };
}

define_encrypt_round!(16, 7, 2);
define_encrypt_round!(24, 8, 3);
define_encrypt_round!(32, 8, 3);
define_encrypt_round!(48, 8, 3);
define_encrypt_round!(64, 8, 3);
