use bytemuck::{Pod, Zeroable};
use std::ops::BitXor;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Zeroable, Pod)]
pub struct U24(u32);

impl U24 {
    const U24_MASK: u32 = 0x00FF_FFFF;
    const U24_BITS: u32 = 24;

    pub fn wrapping_add(self, rhs: Self) -> Self {
        U24(self.0.wrapping_add(rhs.0) & Self::U24_MASK)
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        U24(self.0.wrapping_sub(rhs.0) & Self::U24_MASK)
    }

    pub fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::U24_BITS;
        U24(((self.0 << n) | (self.0 >> (Self::U24_BITS - n))) & Self::U24_MASK)
    }

    pub fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::U24_BITS;
        U24(((self.0 >> n) | (self.0 << (Self::U24_BITS - n))) & Self::U24_MASK)
    }
}

impl BitXor for U24 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        U24(self.0.bitxor(rhs.0) & Self::U24_MASK)
    }
}

impl From<u8> for U24 {
    #[inline(always)]
    fn from(value: u8) -> Self {
        U24((value as u32) & Self::U24_MASK)
    }
}

impl From<u32> for U24 {
    fn from(value: u32) -> Self {
        U24(value & Self::U24_MASK)
    }
}

impl From<i32> for U24 {
    fn from(value: i32) -> Self {
        U24((value as u32) & Self::U24_MASK)
    }
}

impl From<U24> for u32 {
    fn from(value: U24) -> Self {
        value.0
    }
}
