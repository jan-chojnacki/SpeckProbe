#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
use crate::backend::avx2::key::AVX2Key;
#[cfg(all(
    target_arch = "x86_64",
    target_arch = "x86_64",
    target_feature = "avx512bw"
))]
use crate::backend::avx512::key::AVX512Key;
#[cfg(target_arch = "aarch64")]
use crate::backend::neon::key::NEONKey;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use crate::backend::sse2::key::SSE2Key;
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
pub struct KeyIterator {
    current: u64,
    end: u64,
    prefix: [u8; 24],
    prefix_len: usize,
    speck_version: SpeckVersion,
}

impl KeyIterator {
    pub fn new(
        start: u64,
        count: u64,
        prefix: &[u8],
        speck_version: SpeckVersion,
    ) -> Result<Self, KeyIteratorError> {
        let expected = speck_version.prefix_size_bytes();

        let prefix_len = prefix.len();

        if prefix_len != expected {
            return Err(KeyIteratorError::InvalidPrefixLength {
                expected,
                got: prefix_len,
            });
        }

        let mut new_prefix = [0u8; 24];
        new_prefix[..prefix_len].copy_from_slice(prefix);
        let prefix = new_prefix;

        let current = start;

        let end = match start.checked_add(count) {
            None => {
                return Err(KeyIteratorError::InvalidKeyCount { start, count });
            }
            Some(s) => s,
        };

        Ok(Self {
            current,
            end,
            prefix,
            prefix_len,
            speck_version,
        })
    }

    pub fn new_key(&self) -> Key {
        Key::new(&self.prefix[..self.prefix_len], self.current)
    }

    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    #[target_feature(enable = "sse2")]
    pub fn sse2_new_key<const T: usize>(&self) -> SSE2Key<T> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        SSE2Key::new(&self.prefix[..self.prefix_len], v, self.speck_version)
    }

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    #[target_feature(enable = "avx2")]
    pub fn avx2_new_key<const T: usize>(&self) -> AVX2Key<T> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        AVX2Key::new(&self.prefix[..self.prefix_len], v, self.speck_version)
    }

    #[cfg(all(
        target_arch = "x86_64",
        target_arch = "x86_64",
        target_feature = "avx512bw"
    ))]
    #[target_feature(enable = "avx512f")]
    pub fn avx512_new_key<const T: usize>(&self) -> AVX512Key<T> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        AVX512Key::new(&self.prefix[..self.prefix_len], v, self.speck_version)
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_new_key<const T: usize>(&self) -> NEONKey<T> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        NEONKey::new(&self.prefix[..self.prefix_len], v, self.speck_version)
    }

    pub fn next_into(&mut self, out: &mut Key) -> Option<()> {
        if self.current >= self.end {
            return None;
        }

        let v = self.current;
        self.current = self.current.saturating_add(1);

        out.update(v);

        Some(())
    }

    pub fn simd_next_into<const T: usize>(&mut self, out: &mut impl SimdKey<T>) -> Option<()> {
        if self.current >= self.end {
            return None;
        }

        let v = std::array::from_fn(|i| self.current + i as u64);
        self.current = self.current.saturating_add(T as u64);

        out.update(v);

        Some(())
    }
}
