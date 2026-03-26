use engine::api::request::{Operation, SearchRangeRequest};
use engine::domain::block::Block;
use primitive_types::U256;
use speck::SpeckVersion;

pub struct TaskIterator {
    range_start: U256,
    range_end: U256,
    current: U256,
    batch_size: u64,
    speck_version: SpeckVersion,
    data_bytes: Block,
    expected_bytes: Block,
    operation: Operation,
    finished: bool,
}

impl TaskIterator {
    pub fn new(
        range_start: U256,
        range_end: U256,
        batch_size: u64,
        speck_version: SpeckVersion,
        data_bytes: Block,
        expected_bytes: Block,
        operation: Operation,
    ) -> Self {
        assert!(batch_size > 0, "batch_size must be > 0");
        assert!(range_start <= range_end, "range_start must be <= range_end");

        Self {
            range_start: range_start.clone(),
            range_end,
            current: range_start,
            batch_size,
            speck_version,
            data_bytes,
            expected_bytes,
            operation,
            finished: false,
        }
    }

    #[inline]
    fn max_last_offset_for_same_prefix(current: U256) -> u64 {
        u64::MAX - current.low_u64()
    }

    #[inline]
    fn max_last_offset_for_range(current: U256, range_end: U256) -> u64 {
        let diff = range_end - current;

        if diff > U256::from(u64::MAX) {
            u64::MAX
        } else {
            diff.low_u64()
        }
    }

    #[inline]
    fn max_last_offset_for_batch(batch_size: u64) -> u64 {
        debug_assert!(batch_size > 0);
        batch_size - 1
    }

    #[inline]
    fn prefix_from_current(current: U256, speck_version: &SpeckVersion) -> Vec<u8> {
        let prefix_len = speck_version.prefix_size_bytes();
        if prefix_len == 0 {
            return Vec::new();
        }

        let bytes = current.to_little_endian();

        bytes[8..8 + prefix_len].to_vec()
    }
}

impl Iterator for TaskIterator {
    type Item = SearchRangeRequest;
    //TODO dodać testy sprawdzające czy nie pomijana jest żaden klucz
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.current > self.range_end {
            return None;
        }

        let start_low = self.current.low_u64();

        let max_by_batch = Self::max_last_offset_for_batch(self.batch_size);
        let max_by_range = Self::max_last_offset_for_range(self.current, self.range_end);
        let max_by_prefix = Self::max_last_offset_for_same_prefix(self.current);

        let last_offset = max_by_batch.min(max_by_range).min(max_by_prefix);

        let prefix = Self::prefix_from_current(self.current, &self.speck_version);

        let request = SearchRangeRequest::new(
            self.speck_version,
            start_low,
            last_offset,
            prefix,
            self.data_bytes.clone(),
            self.expected_bytes.clone(),
            self.operation,
        );

        let consumed = U256::from(last_offset) + U256::from(1u8);

        match self.current.checked_add(consumed) {
            Some(next) => {
                self.current = next;
            }
            None => {
                self.finished = true;
            }
        }

        Some(request)
    }
}
