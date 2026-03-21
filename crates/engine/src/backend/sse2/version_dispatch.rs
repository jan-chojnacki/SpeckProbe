use crate::SearchEngineBackendError;
use crate::api::request::{Operation, SearchRangeRequest};
use crate::backend::sse2::comparator::{
    sse2_block_compare_u16, sse2_block_compare_u32, sse2_block_compare_u64,
};
use crate::backend::sse2::converter::{
    sse2_u16x2_block_to_vec, sse2_u32x2_block_to_vec, sse2_u64x2_block_to_vec,
};
use crate::backend::sse2::runner::sse2_run_search;
use crate::domain::key::Key;
use speck::{
    sse2_decrypt_block_32_64, sse2_decrypt_block_48_72, sse2_decrypt_block_48_96,
    sse2_decrypt_block_64_96, sse2_decrypt_block_64_128, sse2_decrypt_block_96_96,
    sse2_decrypt_block_96_144, sse2_decrypt_block_128_128, sse2_decrypt_block_128_192,
    sse2_decrypt_block_128_256, sse2_encrypt_block_32_64, sse2_encrypt_block_48_72,
    sse2_encrypt_block_48_96, sse2_encrypt_block_64_96, sse2_encrypt_block_64_128,
    sse2_encrypt_block_96_96, sse2_encrypt_block_96_144, sse2_encrypt_block_128_128,
    sse2_encrypt_block_128_192, sse2_encrypt_block_128_256,
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_32_64(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u16x2_block_to_vec(req.data_bytes.as_u16x2_le()?);
    let expected = sse2_u16x2_block_to_vec(req.expected_bytes.as_u16x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u16(e, b, key, out),
            |k| k.sse2_u16x4_key(),
            |d, k| sse2_encrypt_block_32_64(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u16(e, b, key, out),
            |k| k.sse2_u16x4_key(),
            |d, k| sse2_decrypt_block_32_64(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_48_72(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u32x2_block_to_vec(req.data_bytes.as_u24x2_le()?);
    let expected = sse2_u32x2_block_to_vec(req.expected_bytes.as_u24x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u32(e, b, key, out),
            |k| k.sse2_u24x3_key(),
            |d, k| sse2_encrypt_block_48_72(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u32(e, b, key, out),
            |k| k.sse2_u24x3_key(),
            |d, k| sse2_decrypt_block_48_72(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_48_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u32x2_block_to_vec(req.data_bytes.as_u24x2_le()?);
    let expected = sse2_u32x2_block_to_vec(req.expected_bytes.as_u24x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u32(e, b, key, out),
            |k| k.sse2_u24x4_key(),
            |d, k| sse2_encrypt_block_48_96(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u32(e, b, key, out),
            |k| k.sse2_u24x4_key(),
            |d, k| sse2_decrypt_block_48_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_64_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u32x2_block_to_vec(req.data_bytes.as_u32x2_le()?);
    let expected = sse2_u32x2_block_to_vec(req.expected_bytes.as_u32x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u32(e, b, key, out),
            |k| k.sse2_u32x3_key(),
            |d, k| sse2_encrypt_block_64_96(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u32(e, b, key, out),
            |k| k.sse2_u32x3_key(),
            |d, k| sse2_decrypt_block_64_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_64_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u32x2_block_to_vec(req.data_bytes.as_u32x2_le()?);
    let expected = sse2_u32x2_block_to_vec(req.expected_bytes.as_u32x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u32(e, b, key, out),
            |k| k.sse2_u32x4_key(),
            |d, k| sse2_encrypt_block_64_128(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u32(e, b, key, out),
            |k| k.sse2_u32x4_key(),
            |d, k| sse2_decrypt_block_64_128(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_96_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u64x2_block_to_vec(req.data_bytes.as_u48x2_le()?);
    let expected = sse2_u64x2_block_to_vec(req.expected_bytes.as_u48x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u48x2_key(),
            |d, k| sse2_encrypt_block_96_96(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u48x2_key(),
            |d, k| sse2_decrypt_block_96_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_96_144(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u64x2_block_to_vec(req.data_bytes.as_u48x2_le()?);
    let expected = sse2_u64x2_block_to_vec(req.expected_bytes.as_u48x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u48x3_key(),
            |d, k| sse2_encrypt_block_96_144(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u48x3_key(),
            |d, k| sse2_decrypt_block_96_144(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_128_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = sse2_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u64x2_key(),
            |d, k| sse2_encrypt_block_128_128(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u64x2_key(),
            |d, k| sse2_decrypt_block_128_128(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_128_192(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = sse2_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u64x3_key(),
            |d, k| sse2_encrypt_block_128_192(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u64x3_key(),
            |d, k| sse2_decrypt_block_128_192(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[target_feature(enable = "sse2")]
pub fn sse2_search_128_256(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = sse2_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = sse2_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u64x4_key(),
            |d, k| sse2_encrypt_block_128_256(d, k),
        ),
        Operation::Decrypt => sse2_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| sse2_block_compare_u64(e, b, key, out),
            |k| k.sse2_u64x4_key(),
            |d, k| sse2_decrypt_block_128_256(d, k),
        ),
    }
}
