use thiserror::Error;
use crate::key::Key;
use crate::speck_version::SpeckVersion;

#[derive(Debug, Error)]
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
}

impl KeyIterator {
    pub fn new(
        start: u64,
        count: u64,
        prefix: &[u8],
        version: &SpeckVersion,
    ) -> Result<Self, KeyIteratorError> {
        let expected = version.prefix_size_bytes();

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
        })
    }

    pub fn new_key(&self) -> Key {
        Key::new(&self.prefix, self.prefix_len, self.current)
    }

    pub fn next_into(&mut self, out: &mut Key) -> Option<()> {
        if self.current >= self.end {
            return None;
        }

        let v = self.current;
        self.current += 1;

        out.update(v);

        Some(())
    }
}