use crate::SearchEngineBackendError;
use crate::api::request::SearchRangeRequest;
use crate::backend::neon::key::NEONKey;
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_run_search<FK, FW, FC, V, const T: usize, const W: usize>(
    req: &SearchRangeRequest,
    data: [V; 2],
    expected: &[V; 2],
    comparator: FC,
    key_words: FK,
    cipher: FW,
) -> Result<Vec<Key>, SearchEngineBackendError>
where
    V: Copy,
    FK: Fn(&NEONKey<T>) -> [V; W],
    FW: Fn([V; 2], [V; W]) -> [V; 2],
    FC: Fn(&[V; 2], &[V; 2], &NEONKey<T>, &mut Vec<Key>),
{
    let mut iterator =
        KeyIterator::new(req.start_key, req.key_count, &req.prefix, req.speck_version)?;

    let mut key: NEONKey<T> = iterator.neon_new_key();
    let mut results = Vec::with_capacity(16);

    while iterator.simd_next_into(&mut key).is_some() {
        let r = cipher(data, key_words(&key));
        comparator(expected, &r, &key, &mut results);
    }

    Ok(results)
}
