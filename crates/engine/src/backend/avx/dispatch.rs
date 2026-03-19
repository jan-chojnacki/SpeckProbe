use crate::SearchEngineBackendError;
use crate::api::request::SearchRangeRequest;
use crate::api::version::SpeckVersion;
use crate::backend::avx::version_dispatch::{
    search_32_64, search_48_72, search_48_96, search_64_96, search_64_128, search_96_96,
    search_96_144, search_128_128, search_128_192, search_128_256,
};
use crate::domain::key::Key;

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn avx_search_range_impl(
    req: SearchRangeRequest,
) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
    let results = match req.speck_version {
        SpeckVersion::Speck32_64 => search_32_64(&req),
        SpeckVersion::Speck48_72 => search_48_72(&req),
        SpeckVersion::Speck48_96 => search_48_96(&req),
        SpeckVersion::Speck64_96 => search_64_96(&req),
        SpeckVersion::Speck64_128 => search_64_128(&req),
        SpeckVersion::Speck96_96 => search_96_96(&req),
        SpeckVersion::Speck96_144 => search_96_144(&req),
        SpeckVersion::Speck128_128 => search_128_128(&req),
        SpeckVersion::Speck128_192 => search_128_192(&req),
        SpeckVersion::Speck128_256 => search_128_256(&req),
    }?;

    Ok((!results.is_empty()).then_some(results))
}
