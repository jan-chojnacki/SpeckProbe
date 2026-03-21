use crate::api::version::SpeckVersion;
use crate::backend::avx::key_converter::AvxSimdKey;
use crate::domain::key::Key;
use crate::domain::simd_key::SimdKey;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
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

    #[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
    #[target_feature(enable = "avx")]
    pub fn new_avx_key<const T: usize>(&self) -> AvxSimdKey<T> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        AvxSimdKey::new(&self.prefix[..self.prefix_len], v, self.speck_version)
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

    pub fn simd_next_into<const T: usize>(&mut self, out: &mut AvxSimdKey<T>) -> Option<()> {
        if self.current >= self.end {
            return None;
        }

        let v = std::array::from_fn(|i| self.current + i as u64);
        self.current = self.current.saturating_add(T as u64);

        out.update(v);

        Some(())
    }
}
