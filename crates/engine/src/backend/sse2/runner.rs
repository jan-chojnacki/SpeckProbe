use crate::SearchEngineBackendError;
use crate::api::request::SearchRangeRequest;
use crate::backend::sse2::key_converter::SSE2Key;
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;
use std::arch::x86_64::__m128i;

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn run_sse2_search<FK, FW, FC, const T: usize, const W: usize>(
    req: &SearchRangeRequest,
    data: [__m128i; 2],
    expected: &[__m128i; 2],
    comparator: FC,
    key_words: FK,
    cipher: FW,
) -> Result<Vec<Key>, SearchEngineBackendError>
where
    FK: Fn(&SSE2Key<T>) -> [__m128i; W],
    FW: Fn([__m128i; 2], [__m128i; W]) -> [__m128i; 2],
    FC: Fn(&[__m128i; 2], &[__m128i; 2], &SSE2Key<T>, &mut Vec<Key>),
{
    let mut iterator =
        KeyIterator::new(req.start_key, req.key_count, &req.prefix, req.speck_version)?;

    let mut key: SSE2Key<T> = iterator.new_sse2_key();
    let mut results = Vec::with_capacity(16);

    while iterator.simd_next_into(&mut key).is_some() {
        let r = cipher(data, key_words(&key));
        comparator(expected, &r, &key, &mut results);
    }

    Ok(results)
}
