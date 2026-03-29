use bytemuck::{Pod, Zeroable};
use std::ops::BitXor;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Zeroable, Pod)]
pub struct U48(u64);

impl U48 {
    const U48_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
    const U48_BITS: u32 = 48;

    pub fn wrapping_add(self, rhs: Self) -> Self {
        U48(self.0.wrapping_add(rhs.0) & Self::U48_MASK)
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        U48(self.0.wrapping_sub(rhs.0) & Self::U48_MASK)
    }

    pub fn rotate_left(self, n: u32) -> Self {
        let n = n % Self::U48_BITS;
        U48(((self.0 << n) | (self.0 >> (Self::U48_BITS - n))) & Self::U48_MASK)
    }

    pub fn rotate_right(self, n: u32) -> Self {
        let n = n % Self::U48_BITS;
        U48(((self.0 >> n) | (self.0 << (Self::U48_BITS - n))) & Self::U48_MASK)
    }
}

impl BitXor for U48 {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        U48(self.0.bitxor(rhs.0) & Self::U48_MASK)
    }
}

impl From<u8> for U48 {
    #[inline(always)]
    fn from(value: u8) -> Self {
        U48((value as u64) & Self::U48_MASK)
    }
}

impl From<u64> for U48 {
    fn from(value: u64) -> Self {
        U48(value & Self::U48_MASK)
    }
}

impl From<i64> for U48 {
    fn from(value: i64) -> Self {
        U48((value as u64) & Self::U48_MASK)
    }
}

impl From<U48> for u64 {
    fn from(value: U48) -> Self {
        value.0
    }
}
