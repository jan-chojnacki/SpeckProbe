use crate::domain::key::Key;

#[inline(always)]
pub fn block_compare<T, const BYTES: usize, const PREFIX: usize>(
    e: &[T; 2],
    v: &[T; 2],
    key: &Key<BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) where
    T: PartialEq,
{
    if v[0] == e[0] && v[1] == e[1] {
        out.push(*key);
    }
}
