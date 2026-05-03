#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Task<T, const BYTES: usize, const PREFIX: usize>
where
    T: Copy + Clone,
{
    pub prefix: [u8; PREFIX],
    pub start: u64,
    pub end: u64,
    pub data: [T; 2],
    pub expected: [T; 2],
}
