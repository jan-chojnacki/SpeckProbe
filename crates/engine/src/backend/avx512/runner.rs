use crate::SearchEngineBackendError;
use crate::api::request::SearchRangeRequest;
use crate::backend::avx512::key::AVX512Key;
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;
use std::arch::x86_64::__m512i;

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_run_search<FK, FW, FC, const T: usize, const W: usize>(
    req: &SearchRangeRequest,
    data: [__m512i; 2],
    expected: &[__m512i; 2],
    comparator: FC,
    key_words: FK,
    cipher: FW,
) -> Result<Vec<Key>, SearchEngineBackendError>
where
    FK: Fn(&AVX512Key<T>) -> [__m512i; W],
    FW: Fn([__m512i; 2], [__m512i; W]) -> [__m512i; 2],
    FC: Fn(&[__m512i; 2], &[__m512i; 2], &AVX512Key<T>, &mut Vec<Key>),
{
    let mut iterator =
        KeyIterator::new(req.start_key, req.key_count, &req.prefix, req.speck_version)?;

    let mut key: AVX512Key<T> = iterator.avx512_new_key();
    let mut results = Vec::with_capacity(16);

    while iterator.simd_next_into(&mut key).is_some() {
        let r = cipher(data, key_words(&key));
        comparator(expected, &r, &key, &mut results);
    }

    Ok(results)
}
