use crate::word_ty;
use crate::constants::*;
use crate::operations::*;
use paste::paste;

macro_rules! define_encrypt_round {
    ($w:literal) => {
        paste! {
            #[inline(always)]
            pub fn [<encrypt_round_ $w>](
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

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::{
    uint16x8_t, uint32x4_t, uint64x2_t
};
#[cfg(target_arch = "aarch64")]
use crate::neon_word_ty;

macro_rules! define_neon_encrypt_round {
    ($w:literal) => {
        paste! {
            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_encrypt_round_ $w>](
                x: &mut neon_word_ty!($w),
                y: &mut neon_word_ty!($w),
                k: neon_word_ty!($w),
            ) {
                let xr = [<neon_ror_alpha_u $w>](*x);
                let s  = [<neon_add_u $w>](xr, *y);
                *x     = [<neon_xor_u $w>](s, k);

                let yl = [<neon_rol_beta_u $w>](*y);
                *y     = [<neon_xor_u $w>](yl, *x);
            }
        }
    };
}

define_neon_encrypt_round!(16);
define_neon_encrypt_round!(32);
define_neon_encrypt_round!(64);