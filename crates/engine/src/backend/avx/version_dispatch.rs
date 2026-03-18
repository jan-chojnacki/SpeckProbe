use crate::SearchEngineBackendError;
use crate::api::request::{Operation, SearchRangeRequest};
use crate::backend::avx::block_converter::{
    u16x2_block_to_avx_vec, u24x2_block_to_avx_vec, u32x2_block_to_avx_vec, u48x2_block_to_avx_vec,
    u64x2_block_to_avx_vec,
};
use crate::backend::avx::comparator::block_compare;
use crate::backend::avx::key_converter::{
    u16x4_key_to_avx_vec, u24x3_key_to_avx_vec, u24x4_key_to_avx_vec, u32x3_key_to_avx_vec,
    u32x4_key_to_avx_vec, u48x2_key_to_avx_vec, u48x3_key_to_avx_vec, u64x2_key_to_avx_vec,
    u64x3_key_to_avx_vec, u64x4_key_to_avx_vec,
};
use crate::backend::avx::runner::run_avx_search;
use crate::domain::key::Key;
use speck::{
    avx_decrypt_block_32_64, avx_decrypt_block_48_72, avx_decrypt_block_48_96,
    avx_decrypt_block_64_96, avx_decrypt_block_64_128, avx_decrypt_block_96_96,
    avx_decrypt_block_96_144, avx_decrypt_block_128_128, avx_decrypt_block_128_192,
    avx_decrypt_block_128_256, avx_encrypt_block_32_64, avx_encrypt_block_48_72,
    avx_encrypt_block_48_96, avx_encrypt_block_64_96, avx_encrypt_block_64_128,
    avx_encrypt_block_96_96, avx_encrypt_block_96_144, avx_encrypt_block_128_128,
    avx_encrypt_block_128_192, avx_encrypt_block_128_256,
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_32_64(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u16x2_block_to_avx_vec(req.data_bytes.as_u16x2_le()?);
    let expected = req.expected_bytes.as_u16x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u16, 8>(e, b),
            |k| u16x4_key_to_avx_vec(k.as_u16x4_le()),
            |d, k| avx_encrypt_block_32_64(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u16, 8>(e, b),
            |k| u16x4_key_to_avx_vec(k.as_u16x4_le()),
            |d, k| avx_decrypt_block_32_64(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_48_72(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u24x2_block_to_avx_vec(req.data_bytes.as_u24x2_le()?);
    let expected = req.expected_bytes.as_u24x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u32, 4>(e, b),
            |k| u24x3_key_to_avx_vec(k.as_u24x3_le()),
            |d, k| avx_encrypt_block_48_72(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u32, 4>(e, b),
            |k| u24x3_key_to_avx_vec(k.as_u24x3_le()),
            |d, k| avx_decrypt_block_48_72(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_48_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u24x2_block_to_avx_vec(req.data_bytes.as_u24x2_le()?);
    let expected = req.expected_bytes.as_u24x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u32, 4>(e, b),
            |k| u24x4_key_to_avx_vec(k.as_u24x4_le()),
            |d, k| avx_encrypt_block_48_96(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u32, 4>(e, b),
            |k| u24x4_key_to_avx_vec(k.as_u24x4_le()),
            |d, k| avx_decrypt_block_48_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_64_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u24x2_block_to_avx_vec(req.data_bytes.as_u24x2_le()?);
    let expected = req.expected_bytes.as_u24x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u32, 4>(e, b),
            |k| u32x3_key_to_avx_vec(k.as_u32x3_le()),
            |d, k| avx_encrypt_block_64_96(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u32, 4>(e, b),
            |k| u32x3_key_to_avx_vec(k.as_u32x3_le()),
            |d, k| avx_decrypt_block_64_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_64_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u32x2_block_to_avx_vec(req.data_bytes.as_u32x2_le()?);
    let expected = req.expected_bytes.as_u32x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u32, 4>(e, b),
            |k| u32x4_key_to_avx_vec(k.as_u32x4_le()),
            |d, k| avx_encrypt_block_64_128(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u32, 4>(e, b),
            |k| u32x4_key_to_avx_vec(k.as_u32x4_le()),
            |d, k| avx_decrypt_block_64_128(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_96_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u48x2_block_to_avx_vec(req.data_bytes.as_u48x2_le()?);
    let expected = req.expected_bytes.as_u48x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u48x2_key_to_avx_vec(k.as_u48x2_le()),
            |d, k| avx_encrypt_block_96_96(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u48x2_key_to_avx_vec(k.as_u48x2_le()),
            |d, k| avx_decrypt_block_96_96(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_96_144(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u48x2_block_to_avx_vec(req.data_bytes.as_u48x2_le()?);
    let expected = req.expected_bytes.as_u48x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u48x3_key_to_avx_vec(k.as_u48x3_le()),
            |d, k| avx_encrypt_block_96_144(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u48x3_key_to_avx_vec(k.as_u48x3_le()),
            |d, k| avx_decrypt_block_96_144(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_128_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u64x2_block_to_avx_vec(req.data_bytes.as_u64x2_le()?);
    let expected = req.expected_bytes.as_u64x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u64x2_key_to_avx_vec(k.as_u64x2_le()),
            |d, k| avx_encrypt_block_128_128(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u64x2_key_to_avx_vec(k.as_u64x2_le()),
            |d, k| avx_decrypt_block_128_128(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_128_192(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u64x2_block_to_avx_vec(req.data_bytes.as_u64x2_le()?);
    let expected = req.expected_bytes.as_u64x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u64x3_key_to_avx_vec(k.as_u64x3_le()),
            |d, k| avx_encrypt_block_128_192(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u64x3_key_to_avx_vec(k.as_u64x3_le()),
            |d, k| avx_decrypt_block_128_192(d, k),
        ),
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn search_128_256(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = u64x2_block_to_avx_vec(req.data_bytes.as_u64x2_le()?);
    let expected = req.expected_bytes.as_u64x2_le()?;

    match req.operation {
        Operation::Encrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u64x4_key_to_avx_vec(k.as_u64x4_le()),
            |d, k| avx_encrypt_block_128_256(d, k),
        ),
        Operation::Decrypt => run_avx_search(
            req,
            data,
            expected,
            |e, b| block_compare::<u64, 2>(e, b),
            |k| u64x4_key_to_avx_vec(k.as_u64x4_le()),
            |d, k| avx_decrypt_block_128_256(d, k),
        ),
    }
}
