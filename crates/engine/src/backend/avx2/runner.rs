use crate::SearchEngineBackendError;
use crate::api::request::SearchRangeRequest;
use crate::backend::avx2::key::AVX2Key;
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;
use std::arch::x86_64::__m256i;

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[target_feature(enable = "avx2")]
pub fn avx2_run_search<FK, FW, FC, const T: usize, const W: usize>(
    req: &SearchRangeRequest,
    data: [__m256i; 2],
    expected: &[__m256i; 2],
    comparator: FC,
    key_words: FK,
    cipher: FW,
) -> Result<Vec<Key>, SearchEngineBackendError>
where
    FK: Fn(&AVX2Key<T>) -> [__m256i; W],
    FW: Fn([__m256i; 2], [__m256i; W]) -> [__m256i; 2],
    FC: Fn(&[__m256i; 2], &[__m256i; 2], &AVX2Key<T>, &mut Vec<Key>),
{
    let mut iterator =
        KeyIterator::new(req.start_key, req.key_count, &req.prefix, req.speck_version)?;

    let mut key: AVX2Key<T> = iterator.avx2_new_key();
    let mut results = Vec::with_capacity(16);

    while iterator.simd_next_into(&mut key).is_some() {
        let r = cipher(data, key_words(&key));
        comparator(expected, &r, &key, &mut results);
    }

    Ok(results)
}
