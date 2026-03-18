use crate::SearchEngineBackendError;
use crate::api::request::SearchRangeRequest;
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;
use crate::domain::simd_key::SimdKey;
use std::arch::x86_64::__m128i;

pub fn run_avx_search<E, FK, FW, FC, const T: usize, const W: usize>(
    req: &SearchRangeRequest,
    data: [__m128i; 2],
    expected: E,
    comparator: FC,
    key_words: FK,
    cipher: FW,
) -> Result<Vec<Key>, SearchEngineBackendError>
where
    FK: Fn(&SimdKey<T>) -> [__m128i; W],
    FW: Fn([__m128i; 2], [__m128i; W]) -> [__m128i; 2],
    FC: Fn(&E, [__m128i; 2]) -> Option<Vec<usize>>,
{
    let mut iterator = KeyIterator::new(
        req.start_key,
        req.key_count,
        &req.prefix,
        &req.speck_version,
    )?;

    let mut key: SimdKey<T> = iterator.new_simd_key();
    let mut results = Vec::new();

    while iterator.simd_next_into(&mut key).is_some() {
        let r = cipher(data, key_words(&key));
        match comparator(&expected, r) {
            None => {}
            Some(keys) => {
                for k in keys {
                    results.push(key.get(k))
                }
            }
        }
    }

    Ok(results)
}
