#[cfg(target_arch = "aarch64")]
use crate::backend::neon::key::NEONKey;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
use crate::backend::x86_64::avx2::key::AVX2Key;
#[cfg(all(
    target_arch = "x86_64",
    target_arch = "x86_64",
    target_feature = "avx512bw"
))]
use crate::backend::x86_64::avx512::key::AVX512Key;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use crate::backend::x86_64::sse2::key::SSE2Key;
use crate::domain::key::Key;
use crate::domain::simd_key::SimdKey;
use speck::SpeckVersion;
use thiserror::Error;

#[derive(Debug, Clone, Error, Eq, PartialEq)]
pub enum KeyIteratorError {
    #[error("expected {expected} bytes, got {got}")]
    InvalidPrefixLength { expected: usize, got: usize },
    #[error("start ({start}) + count ({count}) overflows end value")]
    InvalidKeyCount { start: u64, count: u64 },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeyIterator<const BYTES: usize, const PREFIX: usize> {
    current: u64,
    end: u64,
    prefix: [u8; PREFIX],
    speck_version: SpeckVersion,
    finished: bool,
}

impl<const BYTES: usize, const PREFIX: usize> KeyIterator<BYTES, PREFIX> {
    #[inline(always)]
    pub fn new(start: u64, end: u64, prefix: [u8; PREFIX], speck_version: SpeckVersion) -> Self {
        assert!(start <= end);

        Self {
            current: start,
            end,
            prefix,
            speck_version,
            finished: false,
        }
    }

    #[inline(always)]
    pub fn new_key(&self) -> Key<BYTES, PREFIX> {
        Key::new(&self.prefix, self.current)
    }

    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    pub fn sse2_new_key<const LANES: usize>(&self) -> SSE2Key<LANES, BYTES, PREFIX> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        SSE2Key::new(&self.prefix, v, self.speck_version)
    }

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    pub fn avx2_new_key<const LANES: usize>(&self) -> AVX2Key<LANES, BYTES, PREFIX> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        AVX2Key::new(&self.prefix, v, self.speck_version)
    }

    #[cfg(all(
        target_arch = "x86_64",
        target_arch = "x86_64",
        target_feature = "avx512bw"
    ))]
    #[target_feature(enable = "avx512f")]
    pub fn avx512_new_key<const LANES: usize>(&self) -> AVX512Key<LANES, BYTES, PREFIX> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        AVX512Key::new(&self.prefix, v, self.speck_version)
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_new_key<const T: usize>(&self) -> NEONKey<T> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        NEONKey::new(&self.prefix[..self.prefix_len], v, self.speck_version)
    }

    #[inline(always)]
    pub fn next_into(&mut self, out: &mut Key<BYTES, PREFIX>) -> Option<()> {
        if self.finished {
            return None;
        }

        out.update(self.current);

        if self.current == self.end {
            self.finished = true;
        } else {
            self.current += 1;
        }

        Some(())
    }

    #[inline(always)]
    pub fn simd_next_into<const LANES: usize>(
        &mut self,
        out: &mut impl SimdKey<LANES>,
    ) -> Option<()> {
        if self.finished {
            return None;
        }

        let v = std::array::from_fn(|i| self.current + i as u64);
        out.update(v);

        if self.current == self.end {
            self.finished = true;
        } else {
            self.current = self.current.saturating_add(LANES as u64);
        }

        Some(())
    }
}
