use crate::word_ty;
use crate::constants::*;
use crate::operations::*;
use paste::paste;

macro_rules! define_decrypt_round {
    ($w:literal) => {
        paste! {
            #[inline(always)]
            pub fn [<decrypt_round_ $w>](
                x: &mut word_ty!($w),
                y: &mut word_ty!($w),
                k: word_ty!($w),
            ) {
                let yx = [<xor_u $w>](*y, *x);
                *y     = [<ror_u $w>](yx, [<BETA_ $w>]);

                let xx = [<xor_u $w>](*x, k);
                let s  = [<sub_u $w>](xx, *y);
                *x     = [<rol_u $w>](s, [<ALPHA_ $w>]);
            }
        }
    };
}

define_decrypt_round!(16);
define_decrypt_round!(24);
define_decrypt_round!(32);
define_decrypt_round!(48);
define_decrypt_round!(64);
