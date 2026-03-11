// use crate::operations::neon_rol_alpha_u16;
// use paste::paste;
// #[cfg(target_arch = "aarch64")]
// use crate::neon_word_ty;
// #[cfg(target_arch = "aarch64")]
// use core::arch::aarch64::{
//     uint16x8_t, uint32x4_t, uint64x2_t,
// };
// 
// #[cfg(target_arch = "aarch64")]
// macro_rules! define_neon_decrypt_round {
//     ($w:literal) => {
//         paste! {
//             #[target_feature(enable = "neon")]
//             pub unsafe fn [<neon_decrypt_round_ $w>](
//                 x: &mut neon_word_ty!($w),
//                 y: &mut neon_word_ty!($w),
//                 k: neon_word_ty!($w),
//             ) {
//                 let yx = [<neon_xor_u $w>](*y, *x);
//                 *y     = [<neon_ror_beta_u $w>](yx);
// 
//                 let xx = [<neon_xor_u $w>](*x, k);
//                 let s  = [<neon_sub_u $w>](xx, *y);
//                 *x     = [<neon_rol_alpha_u $w>](s);
//             }
//         }
//     };
// }
// 
// #[cfg(target_arch = "aarch64")]
// define_neon_decrypt_round!(16);
// 
// #[cfg(target_arch = "aarch64")]
// define_neon_decrypt_round!(32);
// 
// #[cfg(target_arch = "aarch64")]
// define_neon_decrypt_round!(64);