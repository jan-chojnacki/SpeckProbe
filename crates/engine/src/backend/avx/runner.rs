use crate::SearchEngineBackendError;
use crate::api::request::SearchRangeRequest;
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;

pub fn avx_run_search<R, K, FK, FC>(
    req: &SearchRangeRequest,
    data: R,
    expected: R,
    key_words: FK,
    cipher: FC,
) -> Result<Vec<Key>, SearchEngineBackendError>
where
    R: Copy + Eq,
    FK: Fn(&Key) -> K,
    FC: Fn(R, K) -> R,
{
    let mut iterator = KeyIterator::new(
        req.start_key,
        req.key_count,
        &req.prefix,
        &req.speck_version,
    )?;

    let mut key = iterator.new_key();
    let mut results = Vec::new();

    while iterator.next_into(&mut key).is_some() {
        if cipher(data, key_words(&key)) == expected {
            results.push(key);
        }
    }

    Ok(results)
}
