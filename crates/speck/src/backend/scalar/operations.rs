use bytemuck::{Pod, Zeroable};
use std::ops::{Add, BitAnd, BitXor, Sub};

const U24_MASK: u32 = 0x00FF_FFFF;
const U24_BITS: u32 = 24;
const U48_BITS: u32 = 48;
const U48_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Zeroable, Pod)]
pub struct U24(u32);

impl U24 {
    const U24_MASK: u32 = 0x00FF_FFFF;
    const U24_BITS: u32 = 24;

    pub fn wrapping_add(self, rhs: Self) -> Self {
        U24(self.0.wrapping_add(rhs.0) & U24_MASK)
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        U24(self.0.wrapping_sub(rhs.0) & U24_MASK)
    }

    pub fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::U24_BITS;
        U24(((self.0 << n) | (self.0 >> (U24_BITS - n))) & U24_MASK)
    }

    pub fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::U24_BITS;
        U24(((self.0 >> n) | (self.0 << (U24_BITS - n))) & U24_MASK)
    }
}

impl BitXor for U24 {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        U24(self.0.bitxor(rhs.0) & U24_MASK)
    }
}

impl From<u8> for U24 {
    #[inline(always)]
    fn from(value: u8) -> Self {
        U24((value as u32) & U24_MASK)
    }
}

impl From<u32> for U24 {
    fn from(value: u32) -> Self {
        U24(value & U24_MASK)
    }
}

impl From<i32> for U24 {
    fn from(value: i32) -> Self {
        U24((value as u32) & U24_MASK)
    }
}

impl From<U24> for u32 {
    fn from(value: U24) -> Self {
        value.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Zeroable, Pod)]
pub struct U48(u64);

impl U48 {
    const U48_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
    const U48_BITS: u32 = 48;

    pub fn wrapping_add(self, rhs: Self) -> Self {
        U48(self.0.wrapping_add(rhs.0) & U48_MASK)
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        U48(self.0.wrapping_sub(rhs.0) & U48_MASK)
    }

    pub fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::U48_BITS;
        U48(((self.0 << n) | (self.0 >> (U48_BITS - n))) & U48_MASK)
    }

    pub fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::U48_BITS;
        U48(((self.0 >> n) | (self.0 << (U48_BITS - n))) & U48_MASK)
    }
}

impl BitXor for U48 {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        U48(self.0.bitxor(rhs.0) & U48_MASK)
    }
}

impl From<u8> for U48 {
    #[inline(always)]
    fn from(value: u8) -> Self {
        U48((value as u64) & U48_MASK)
    }
}

impl From<u64> for U48 {
    fn from(value: u64) -> Self {
        U48(value & U48_MASK)
    }
}

impl From<i64> for U48 {
    fn from(value: i64) -> Self {
        U48((value as u64) & U48_MASK)
    }
}

impl From<U48> for u64 {
    fn from(value: U48) -> Self {
        value.0
    }
}

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
