use crate::domain::task::Task;

pub struct TaskProducer<T, const BYTES: usize, const PREFIX: usize> {
    current_prefix: [u8; PREFIX],
    end_prefix: [u8; PREFIX],
    finished: bool,
    data: [T; 2],
    expected: [T; 2],
}

impl<T, const BYTES: usize, const PREFIX: usize> TaskProducer<T, BYTES, PREFIX> {
    const SUFFIX: usize = BYTES - PREFIX;
    const END: u64 = match Self::SUFFIX {
        0 => 0,
        1..=7 => (1u64 << (Self::SUFFIX * 8)) - 1,
        _ => u64::MAX,
    };

    #[inline(always)]
    pub fn new(
        start_prefix: [u8; PREFIX],
        end_prefix: [u8; PREFIX],
        data: [T; 2],
        expected: [T; 2],
    ) -> Self {
        Self {
            current_prefix: start_prefix,
            end_prefix,
            finished: false,
            data,
            expected,
        }
    }

    #[inline]
    pub fn add_one(&mut self) -> bool {
        for word in &mut self.current_prefix {
            let (next, carry) = word.overflowing_add(1);
            *word = next;
            if !carry {
                return false;
            }
        }

        true
    }
}

impl<T, const BYTES: usize, const PREFIX: usize> Iterator for TaskProducer<T, BYTES, PREFIX>
where
    T: Copy + Clone,
{
    type Item = Task<T, BYTES, PREFIX>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let end = Self::END;

        let out = Task {
            prefix: self.current_prefix,
            start: 0,
            end,
            data: self.data,
            expected: self.expected,
        };

        if self.current_prefix == self.end_prefix {
            self.finished = true;
        } else {
            self.add_one();
        }

        Some(out)
    }
}
