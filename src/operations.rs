use std::ops::BitXor;
use crate::neon_word_ty;

#[inline(always)]
pub fn ror_u16(v: u16, n: u32) -> u16 {
    v.rotate_right(n)
}

#[inline(always)]
pub fn rol_u16(v: u16, n: u32) -> u16 {
    v.rotate_left(n)
}

#[inline(always)]
pub fn add_u16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}

#[inline(always)]
pub fn sub_u16(a: u16, b: u16) -> u16 {
    a.wrapping_sub(b)
}

#[inline(always)]
pub fn xor_u16(a: u16, b: u16) -> u16 {
    a.bitxor(b)
}

const U24_MASK: u32 = 0x00FF_FFFF;
const U24_BITS: u32 = 24;

#[inline(always)]
pub fn ror_u24(v: u32, n: u32) -> u32 {
    let v = v & U24_MASK;
    let n = n % U24_BITS;
    ((v >> n) | (v << (U24_BITS - n))) & U24_MASK
}

#[inline(always)]
pub fn rol_u24(v: u32, n: u32) -> u32 {
    let v = v & U24_MASK;
    let n = n % U24_BITS;
    ((v << n) | (v >> (U24_BITS - n))) & U24_MASK
}

#[inline(always)]
pub fn add_u24(a: u32, b: u32) -> u32 {
    a.wrapping_add(b) & U24_MASK
}

#[inline(always)]
pub fn sub_u24(a: u32, b: u32) -> u32 {
    a.wrapping_sub(b) & U24_MASK
}

#[inline(always)]
pub fn xor_u24(a: u32, b: u32) -> u32 {
    a.bitxor(b) & U24_MASK
}

#[inline(always)]
pub fn ror_u32(v: u32, n: u32) -> u32 {
    v.rotate_right(n)
}

#[inline(always)]
pub fn rol_u32(v: u32, n: u32) -> u32 {
    v.rotate_left(n)
}

#[inline(always)]
pub fn add_u32(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

#[inline(always)]
pub fn sub_u32(a: u32, b: u32) -> u32 {
    a.wrapping_sub(b)
}

#[inline(always)]
pub fn xor_u32(a: u32, b: u32) -> u32 {
    a.bitxor(b)
}

const U48_BITS: u32 = 48;
const U48_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

#[inline(always)]
pub fn ror_u48(v: u64, n: u32) -> u64 {
    let v = v & U48_MASK;
    let n = n % U48_BITS;
    ((v >> n) | (v << (U48_BITS - n))) & U48_MASK
}

#[inline(always)]
pub fn rol_u48(v: u64, n: u32) -> u64 {
    let v = v & U48_MASK;
    let n = n % U48_BITS;
    ((v << n) | (v >> (U48_BITS - n))) & U48_MASK
}

#[inline(always)]
pub fn add_u48(a: u64, b: u64) -> u64 {
    a.wrapping_add(b) & U48_MASK
}

#[inline(always)]
pub fn sub_u48(a: u64, b: u64) -> u64 {
    a.wrapping_sub(b) & U48_MASK
}

#[inline(always)]
pub fn xor_u48(a: u64, b: u64) -> u64 {
    a.bitxor(b) & U48_MASK
}

#[inline(always)]
pub fn ror_u64(v: u64, n: u32) -> u64 {
    v.rotate_right(n)
}

#[inline(always)]
pub fn rol_u64(v: u64, n: u32) -> u64 {
    v.rotate_left(n)
}

#[inline(always)]
pub fn add_u64(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}

#[inline(always)]
pub fn sub_u64(a: u64, b: u64) -> u64 {
    a.wrapping_sub(b)
}

#[inline(always)]
pub fn xor_u64(a: u64, b: u64) -> u64 {
    a.bitxor(b)
}

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
use paste::paste;
use crate::constants::*;

macro_rules! define_neon_ror {
    ($word:literal) => {
        paste! {
            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_ror_alpha_u $word>](x: neon_word_ty!($word)) -> neon_word_ty!($word) {
                let hi = [<vshrq_n_u $word>]::<{ [<ALPHA_ $word>] as i32 }>(x);
                let lo = [<vshlq_n_u $word>]::<{ $word - [<ALPHA_ $word>] as i32 }>(x);
                [<vorrq_u $word>](hi, lo)
            }

            #[target_feature(enable = "neon")]
            pub unsafe fn [<neon_ror_beta_u $word>](x: neon_word_ty!($word)) -> neon_word_ty!($word) {
                let hi = [<vshrq_n_u $word>]::<{ [<BETA_ $word>] as i32 }>(x);
                let lo = [<vshlq_n_u $word>]::<{ $word - [<BETA_ $word>] as i32 }>(x);
                [<vorrq_u $word>](hi, lo)
            }
        }
    };
}

define_neon_ror!(16);
define_neon_ror!(32);
define_neon_ror!(64);