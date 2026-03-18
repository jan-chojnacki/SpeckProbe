use crate::api::version::SpeckVersion;
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
        Key::new(
            &self.prefix[..self.prefix_len],
            self.prefix_len,
            self.current,
        )
    }

    pub fn new_simd_key<const T: usize>(&self) -> SimdKey<T> {
        let v = std::array::from_fn(|i| self.current + i as u64);
        SimdKey::new(&self.prefix[..self.prefix_len], self.prefix_len, v)
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

    pub fn simd_next_into<const T: usize>(&mut self, out: &mut SimdKey<T>) -> Option<()> {
        if self.current >= self.end {
            return None;
        }

        let v = std::array::from_fn(|i| self.current + i as u64);
        self.current = self.current.saturating_add(T as u64);

        out.update(v);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn assert_simd_matches_scalar_sequence<const T: usize>(
        simd: &SimdKey<T>,
        prefix: &[u8],
        start: u64,
    ) {
        for lane in 0..T {
            assert_eq!(
                simd.as_bytes()[lane],
                Key::new(prefix, prefix.len(), start.saturating_add(lane as u64)).as_bytes()
            );
        }
    }

    #[rstest]
    #[case(SpeckVersion::Speck32_64, vec![])]
    #[case(SpeckVersion::Speck48_72, vec![0xAB])]
    #[case(SpeckVersion::Speck48_96, vec![0xCD; 4])]
    #[case(SpeckVersion::Speck64_128, vec![0xEF; 8])]
    #[case(SpeckVersion::Speck128_192, vec![0x11; 16])]
    #[case(SpeckVersion::Speck128_256, vec![0x22; 24])]
    fn new_accepts_correct_prefix_length_for_version(
        #[case] version: SpeckVersion,
        #[case] prefix: Vec<u8>,
    ) {
        let iter = KeyIterator::new(7, 3, &prefix, &version).unwrap();

        let mut out = iter.new_key();
        assert_eq!(
            out.as_bytes(),
            Key::new(&prefix, prefix.len(), 7).as_bytes()
        );

        assert_eq!(iter.clone().next_into(&mut out), Some(()));
        assert_eq!(
            out.as_bytes(),
            Key::new(&prefix, prefix.len(), 7).as_bytes()
        );
    }

    #[rstest]
    #[case(SpeckVersion::Speck32_64, vec![0xAA], 0, 1)]
    #[case(SpeckVersion::Speck48_72, vec![], 1, 0)]
    #[case(SpeckVersion::Speck64_96, vec![0xAA; 3], 4, 3)]
    #[case(SpeckVersion::Speck96_144, vec![0xAA; 9], 10, 9)]
    #[case(SpeckVersion::Speck128_256, vec![0xAA; 23], 24, 23)]
    fn new_rejects_invalid_prefix_length_for_version(
        #[case] version: SpeckVersion,
        #[case] prefix: Vec<u8>,
        #[case] expected: usize,
        #[case] got: usize,
    ) {
        let err = KeyIterator::new(0, 1, &prefix, &version).unwrap_err();
        assert_eq!(err, KeyIteratorError::InvalidPrefixLength { expected, got });
    }

    #[rstest]
    #[case(0, 0)]
    #[case(10, 0)]
    #[case(u64::MAX - 5, 3)]
    fn new_accepts_non_overflowing_range(#[case] start: u64, #[case] count: u64) {
        let prefix = [0xAA; 8];
        let version = SpeckVersion::Speck64_128;

        let iter = KeyIterator::new(start, count, &prefix, &version);
        assert!(iter.is_ok());
    }

    #[rstest]
    #[case(u64::MAX, 1)]
    #[case(u64::MAX - 1, 2)]
    #[case(42, u64::MAX)]
    fn new_rejects_overflowing_range(#[case] start: u64, #[case] count: u64) {
        let prefix = [0xAA; 8];
        let version = SpeckVersion::Speck64_128;

        let err = KeyIterator::new(start, count, &prefix, &version).unwrap_err();
        assert_eq!(err, KeyIteratorError::InvalidKeyCount { start, count });
    }

    #[rstest]
    #[case(vec![], SpeckVersion::Speck32_64, 0x1122334455667788,
        vec![0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11])]
    #[case(vec![0xAA, 0xBB, 0xCC, 0xDD], SpeckVersion::Speck64_96, 0x0102030405060708,
        vec![0xAA, 0xBB, 0xCC, 0xDD, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])]
    #[case(vec![0x7F; 24], SpeckVersion::Speck128_256, 0x1817161514131211,
        vec![0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
             0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18]
    )]
    fn new_key_builds_key_for_current_value(
        #[case] prefix: Vec<u8>,
        #[case] version: SpeckVersion,
        #[case] start: u64,
        #[case] expected: Vec<u8>,
    ) {
        let iter = KeyIterator::new(start, 1, &prefix, &version).unwrap();
        let key = iter.new_key();

        assert_eq!(key.as_bytes(), expected);
    }

    #[rstest]
    #[case(vec![], SpeckVersion::Speck32_64, 0, 3, vec![
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ])]
    #[case(vec![0x10, 0x20, 0x30, 0x40], SpeckVersion::Speck64_96, 5, 2, vec![
        vec![0x10, 0x20, 0x30, 0x40, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x10, 0x20, 0x30, 0x40, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ])]
    fn next_into_iterates_until_end(
        #[case] prefix: Vec<u8>,
        #[case] version: SpeckVersion,
        #[case] start: u64,
        #[case] count: u64,
        #[case] expected_values: Vec<Vec<u8>>,
    ) {
        let mut iter = KeyIterator::new(start, count, &prefix, &version).unwrap();
        let mut out = iter.new_key();

        for expected in expected_values {
            assert_eq!(iter.next_into(&mut out), Some(()));
            assert_eq!(out.as_bytes(), expected);
        }

        assert_eq!(iter.next_into(&mut out), None);
        assert_eq!(iter.next_into(&mut out), None);
    }

    #[test]
    fn next_into_returns_none_for_empty_iterator() {
        let prefix = [0xAA; 8];
        let version = SpeckVersion::Speck64_128;

        let mut iter = KeyIterator::new(123, 0, &prefix, &version).unwrap();
        let mut out = iter.new_key();
        let before = out.to_vec();

        assert_eq!(iter.next_into(&mut out), None);
        assert_eq!(out.to_vec(), before);
    }

    #[rstest]
    #[case(vec![], SpeckVersion::Speck32_64, 0)]
    #[case(vec![0xAA, 0xBB, 0xCC, 0xDD], SpeckVersion::Speck64_96, 5)]
    #[case(vec![0x7F; 24], SpeckVersion::Speck128_256, 0x1122334455667788)]
    fn new_simd_key_builds_lanes_for_current_value(
        #[case] prefix: Vec<u8>,
        #[case] version: SpeckVersion,
        #[case] start: u64,
    ) {
        let iter = KeyIterator::new(start, 8, &prefix, &version).unwrap();
        let simd = iter.new_simd_key::<4>();

        assert_simd_matches_scalar_sequence(&simd, &prefix, start);
    }

    #[rstest]
    #[case(vec![], SpeckVersion::Speck32_64, 0, 8)]
    #[case(vec![0x10, 0x20, 0x30, 0x40], SpeckVersion::Speck64_96, 5, 8)]
    fn simd_next_into_iterates_in_simd_chunks(
        #[case] prefix: Vec<u8>,
        #[case] version: SpeckVersion,
        #[case] start: u64,
        #[case] count: u64,
    ) {
        let mut iter = KeyIterator::new(start, count, &prefix, &version).unwrap();
        let mut out = iter.new_simd_key::<4>();

        assert_eq!(iter.simd_next_into(&mut out), Some(()));
        assert_simd_matches_scalar_sequence(&out, &prefix, start);

        assert_eq!(iter.simd_next_into(&mut out), Some(()));
        assert_simd_matches_scalar_sequence(&out, &prefix, start + 4);

        assert_eq!(iter.simd_next_into(&mut out), None);
        assert_eq!(iter.simd_next_into(&mut out), None);
    }

    #[test]
    fn simd_next_into_returns_none_for_empty_iterator() {
        let prefix = [0xAA; 8];
        let version = SpeckVersion::Speck64_128;

        let mut iter = KeyIterator::new(123, 0, &prefix, &version).unwrap();
        let mut out = iter.new_simd_key::<4>();
        let before = out.to_vec();

        assert_eq!(iter.simd_next_into(&mut out), None);
        assert_eq!(out.to_vec(), before);
    }
}
