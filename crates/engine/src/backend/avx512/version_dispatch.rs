use crate::SearchEngineBackendError;
use crate::api::request::{Operation, SearchRangeRequest};
use crate::backend::avx512::comparator::{
    avx512_block_compare_u16, avx512_block_compare_u32, avx512_block_compare_u64,
};
use crate::backend::avx512::converter::{
    avx512_u16x2_block_to_vec, avx512_u32x2_block_to_vec, avx512_u64x2_block_to_vec,
};
use crate::backend::avx512::runner::avx512_run_search;
use crate::domain::key::Key;
use speck::{
    avx512_decrypt_block_32_64, avx512_decrypt_block_48_72, avx512_decrypt_block_48_96,
    avx512_decrypt_block_64_96, avx512_decrypt_block_64_128, avx512_decrypt_block_96_96,
    avx512_decrypt_block_96_144, avx512_decrypt_block_128_128, avx512_decrypt_block_128_192,
    avx512_decrypt_block_128_256, avx512_encrypt_block_32_64, avx512_encrypt_block_48_72,
    avx512_encrypt_block_48_96, avx512_encrypt_block_64_96, avx512_encrypt_block_64_128,
    avx512_encrypt_block_96_96, avx512_encrypt_block_96_144, avx512_encrypt_block_128_128,
    avx512_encrypt_block_128_192, avx512_encrypt_block_128_256,
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
#[target_feature(enable = "avx512bw")]
pub fn avx512_search_32_64(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u16x2_block_to_vec(req.data_bytes.as_u16x2_le()?);
    let expected = avx512_u16x2_block_to_vec(req.expected_bytes.as_u16x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u16(e, b, key, out),
            |k| k.avx512_u16x4_key(),
            |d, k| avx512_encrypt_block_32_64(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u16(e, b, key, out),
            |k| k.avx512_u16x4_key(),
            |d, k| avx512_decrypt_block_32_64(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_48_72(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u32x2_block_to_vec(req.data_bytes.as_u24x2_le()?);
    let expected = avx512_u32x2_block_to_vec(req.expected_bytes.as_u24x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u32(e, b, key, out),
            |k| k.avx512_u24x3_key(),
            |d, k| avx512_encrypt_block_48_72(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u32(e, b, key, out),
            |k| k.avx512_u24x3_key(),
            |d, k| avx512_decrypt_block_48_72(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_48_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u32x2_block_to_vec(req.data_bytes.as_u24x2_le()?);
    let expected = avx512_u32x2_block_to_vec(req.expected_bytes.as_u24x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u32(e, b, key, out),
            |k| k.avx512_u24x4_key(),
            |d, k| avx512_encrypt_block_48_96(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u32(e, b, key, out),
            |k| k.avx512_u24x4_key(),
            |d, k| avx512_decrypt_block_48_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_64_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u32x2_block_to_vec(req.data_bytes.as_u32x2_le()?);
    let expected = avx512_u32x2_block_to_vec(req.expected_bytes.as_u32x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u32(e, b, key, out),
            |k| k.avx512_u32x3_key(),
            |d, k| avx512_encrypt_block_64_96(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u32(e, b, key, out),
            |k| k.avx512_u32x3_key(),
            |d, k| avx512_decrypt_block_64_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_64_128(
    req: &SearchRangeRequest,
) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u32x2_block_to_vec(req.data_bytes.as_u32x2_le()?);
    let expected = avx512_u32x2_block_to_vec(req.expected_bytes.as_u32x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u32(e, b, key, out),
            |k| k.avx512_u32x4_key(),
            |d, k| avx512_encrypt_block_64_128(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u32(e, b, key, out),
            |k| k.avx512_u32x4_key(),
            |d, k| avx512_decrypt_block_64_128(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_96_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u64x2_block_to_vec(req.data_bytes.as_u48x2_le()?);
    let expected = avx512_u64x2_block_to_vec(req.expected_bytes.as_u48x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u48x2_key(),
            |d, k| avx512_encrypt_block_96_96(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u48x2_key(),
            |d, k| avx512_decrypt_block_96_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_96_144(
    req: &SearchRangeRequest,
) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u64x2_block_to_vec(req.data_bytes.as_u48x2_le()?);
    let expected = avx512_u64x2_block_to_vec(req.expected_bytes.as_u48x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u48x3_key(),
            |d, k| avx512_encrypt_block_96_144(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u48x3_key(),
            |d, k| avx512_decrypt_block_96_144(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_128_128(
    req: &SearchRangeRequest,
) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = avx512_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u64x2_key(),
            |d, k| avx512_encrypt_block_128_128(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u64x2_key(),
            |d, k| avx512_decrypt_block_128_128(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_128_192(
    req: &SearchRangeRequest,
) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = avx512_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u64x3_key(),
            |d, k| avx512_encrypt_block_128_192(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u64x3_key(),
            |d, k| avx512_decrypt_block_128_192(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_search_128_256(
    req: &SearchRangeRequest,
) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = avx512_u64x2_block_to_vec(req.data_bytes.as_u64x2_le()?);
    let expected = avx512_u64x2_block_to_vec(req.expected_bytes.as_u64x2_le()?);

    match req.operation {
        Operation::Encrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u64x4_key(),
            |d, k| avx512_encrypt_block_128_256(d, k),
        ),
        Operation::Decrypt => avx512_run_search(
            req,
            data,
            &expected,
            |e, b, key, out| avx512_block_compare_u64(e, b, key, out),
            |k| k.avx512_u64x4_key(),
            |d, k| avx512_decrypt_block_128_256(d, k),
        ),
    }
}
