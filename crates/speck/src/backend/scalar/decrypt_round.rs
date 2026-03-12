use crate::backend::scalar::operaions::*;
use crate::backend::scalar::word_ty;
use paste::paste;

macro_rules! define_decrypt_round {
    ($w:literal, $alpha:literal, $beta:literal) => {
        paste! {
            #[inline(always)]
            pub fn [<decrypt_round_ $w>](
                x: &mut word_ty!($w),
                y: &mut word_ty!($w),
                k: word_ty!($w),
            ) {
                let yx = [<xor_u $w>](*y, *x);
                *y     = [<ror_u $w>](yx, $beta);

                let xx = [<xor_u $w>](*x, k);
                let s  = [<sub_u $w>](xx, *y);
                *x     = [<rol_u $w>](s, $alpha);
            }
        }
    };
}

define_decrypt_round!(16, 7, 2);
define_decrypt_round!(24, 8, 3);
define_decrypt_round!(32, 8, 3);
define_decrypt_round!(48, 8, 3);
define_decrypt_round!(64, 8, 3);
