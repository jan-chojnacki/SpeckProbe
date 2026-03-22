use crate::SearchEngineBackendError;
use crate::api::request::SearchRangeRequest;
use crate::api::version::SpeckVersion;
use crate::backend::neon::version_dispatch::{
    neon_search_32_64, neon_search_48_72, neon_search_48_96, neon_search_64_96, neon_search_64_128,
    neon_search_96_96, neon_search_96_144, neon_search_128_128, neon_search_128_192,
    neon_search_128_256,
};
use crate::domain::key::Key;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_range_impl(
    req: SearchRangeRequest,
) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
    let results = match req.speck_version {
        SpeckVersion::Speck32_64 => neon_search_32_64(&req),
        SpeckVersion::Speck48_72 => neon_search_48_72(&req),
        SpeckVersion::Speck48_96 => neon_search_48_96(&req),
        SpeckVersion::Speck64_96 => neon_search_64_96(&req),
        SpeckVersion::Speck64_128 => neon_search_64_128(&req),
        SpeckVersion::Speck96_96 => neon_search_96_96(&req),
        SpeckVersion::Speck96_144 => neon_search_96_144(&req),
        SpeckVersion::Speck128_128 => neon_search_128_128(&req),
        SpeckVersion::Speck128_192 => neon_search_128_192(&req),
        SpeckVersion::Speck128_256 => neon_search_128_256(&req),
    }?;

    Ok((!results.is_empty()).then_some(results))
}
