use crate::SearchEngineBackendError;
use crate::api::request::{Operation, SearchRangeRequest};
use crate::backend::neon::comparator::{
    neon_block_compare_u16, neon_block_compare_u32, neon_block_compare_u64,
};
use crate::backend::neon::converter::{
    neon_u16x2_block_to_vec, neon_u32x2_block_to_vec, neon_u64x2_block_to_vec,
};
use crate::backend::neon::runner::neon_run_search;
use crate::domain::key::Key;
use speck::{
    neon_decrypt_block_32_64, neon_decrypt_block_48_72, neon_decrypt_block_48_96,
    neon_decrypt_block_64_96, neon_decrypt_block_64_128, neon_decrypt_block_96_96,
    neon_decrypt_block_96_144, neon_decrypt_block_128_128, neon_decrypt_block_128_192,
    neon_decrypt_block_128_256, neon_encrypt_block_32_64, neon_encrypt_block_48_72,
    neon_encrypt_block_48_96, neon_encrypt_block_64_96, neon_encrypt_block_64_128,
    neon_encrypt_block_96_96, neon_encrypt_block_96_144, neon_encrypt_block_128_128,
    neon_encrypt_block_128_192, neon_encrypt_block_128_256,
};

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_32_64(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u16x2_block_to_vec(req.data_bytes.as_u16x2_le()?);
    let expected = neon_u16x2_block_to_vec(req.expected_bytes.as_u16x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u16(e, b, key, out),
            |k| k.neon_u16x4_key(),
            |d, k| neon_encrypt_block_32_64(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u16(e, b, key, out),
            |k| k.neon_u16x4_key(),
            |d, k| neon_decrypt_block_32_64(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_48_72(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u32x2_block_to_vec(req.data_bytes.as_u24x2_le()?);
    let expected = neon_u32x2_block_to_vec(req.expected_bytes.as_u24x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u32(e, b, key, out),
            |k| k.neon_u24x3_key(),
            |d, k| neon_encrypt_block_48_72(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u32(e, b, key, out),
            |k| k.neon_u24x3_key(),
            |d, k| neon_decrypt_block_48_72(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_48_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u32x2_block_to_vec(req.data_bytes.as_u24x2_le()?);
    let expected = neon_u32x2_block_to_vec(req.expected_bytes.as_u24x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u32(e, b, key, out),
            |k| k.neon_u24x4_key(),
            |d, k| neon_encrypt_block_48_96(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u32(e, b, key, out),
            |k| k.neon_u24x4_key(),
            |d, k| neon_decrypt_block_48_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_64_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u32x2_block_to_vec(req.data_bytes.as_u32x2_le()?);
    let expected = neon_u32x2_block_to_vec(req.expected_bytes.as_u32x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u32(e, b, key, out),
            |k| k.neon_u32x3_key(),
            |d, k| neon_encrypt_block_64_96(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u32(e, b, key, out),
            |k| k.neon_u32x3_key(),
            |d, k| neon_decrypt_block_64_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_64_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u32x2_block_to_vec(req.data_bytes.as_u32x2_le()?);
    let expected = neon_u32x2_block_to_vec(req.expected_bytes.as_u32x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u32(e, b, key, out),
            |k| k.neon_u32x4_key(),
            |d, k| neon_encrypt_block_64_128(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u32(e, b, key, out),
            |k| k.neon_u32x4_key(),
            |d, k| neon_decrypt_block_64_128(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_96_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u64x2_block_to_vec(req.data_bytes.as_u48x2_le()?);
    let expected = neon_u64x2_block_to_vec(req.expected_bytes.as_u48x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u48x2_key(),
            |d, k| neon_encrypt_block_96_96(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u48x2_key(),
            |d, k| neon_decrypt_block_96_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_96_144(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u64x2_block_to_vec(req.data_bytes.as_u48x2_le()?);
    let expected = neon_u64x2_block_to_vec(req.expected_bytes.as_u48x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u48x3_key(),
            |d, k| neon_encrypt_block_96_144(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u48x3_key(),
            |d, k| neon_decrypt_block_96_144(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_128_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = neon_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u64x2_key(),
            |d, k| neon_encrypt_block_128_128(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u64x2_key(),
            |d, k| neon_decrypt_block_128_128(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_128_192(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = neon_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u64x3_key(),
            |d, k| neon_encrypt_block_128_192(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u64x3_key(),
            |d, k| neon_decrypt_block_128_192(d, k),
        ),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
#[target_feature(enable = "neon")]
pub fn neon_search_128_256(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = neon_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = neon_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u64x4_key(),
            |d, k| neon_encrypt_block_128_256(d, k),
        ),
        Operation::Decrypt => neon_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| neon_block_compare_u64(e, b, key, out),
            |k| k.neon_u64x4_key(),
            |d, k| neon_decrypt_block_128_256(d, k),
        ),
    }
}
