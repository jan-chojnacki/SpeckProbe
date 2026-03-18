use crate::SearchEngineBackendError;
use crate::api::request::{Operation, SearchRangeRequest};
use crate::backend::scalar::runner::run_search;
use crate::domain::key::Key;
use speck::{
    decrypt_block_32_64, decrypt_block_48_72, decrypt_block_48_96, decrypt_block_64_96,
    decrypt_block_64_128, decrypt_block_96_96, decrypt_block_96_144, decrypt_block_128_128,
    decrypt_block_128_192, decrypt_block_128_256, encrypt_block_32_64, encrypt_block_48_72,
    encrypt_block_48_96, encrypt_block_64_96, encrypt_block_64_128, encrypt_block_96_96,
    encrypt_block_96_144, encrypt_block_128_128, encrypt_block_128_192, encrypt_block_128_256,
};

pub fn search_32_64(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u16x2_le()?;
    let expected = req.expected_bytes.as_u16x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u16x4_le(),
            encrypt_block_32_64,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u16x4_le(),
            decrypt_block_32_64,
        ),
    }
}

pub fn search_48_72(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u24x2_le()?;
    let expected = req.expected_bytes.as_u24x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u24x3_le(),
            encrypt_block_48_72,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u24x3_le(),
            decrypt_block_48_72,
        ),
    }
}

pub fn search_48_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u24x2_le()?;
    let expected = req.expected_bytes.as_u24x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u24x4_le(),
            encrypt_block_48_96,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u24x4_le(),
            decrypt_block_48_96,
        ),
    }
}

pub fn search_64_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u24x2_le()?;
    let expected = req.expected_bytes.as_u24x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u32x3_le(),
            encrypt_block_64_96,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u32x3_le(),
            decrypt_block_64_96,
        ),
    }
}

pub fn search_64_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u32x2_le()?;
    let expected = req.expected_bytes.as_u32x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u32x4_le(),
            encrypt_block_64_128,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u32x4_le(),
            decrypt_block_64_128,
        ),
    }
}

pub fn search_96_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u48x2_le()?;
    let expected = req.expected_bytes.as_u48x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u48x2_le(),
            encrypt_block_96_96,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u48x2_le(),
            decrypt_block_96_96,
        ),
    }
}

pub fn search_96_144(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u48x2_le()?;
    let expected = req.expected_bytes.as_u48x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u48x3_le(),
            encrypt_block_96_144,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u48x3_le(),
            decrypt_block_96_144,
        ),
    }
}

pub fn search_128_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u64x2_le()?;
    let expected = req.expected_bytes.as_u64x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u64x2_le(),
            encrypt_block_128_128,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u64x2_le(),
            decrypt_block_128_128,
        ),
    }
}

pub fn search_128_192(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u64x2_le()?;
    let expected = req.expected_bytes.as_u64x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u64x3_le(),
            encrypt_block_128_192,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u64x3_le(),
            decrypt_block_128_192,
        ),
    }
}

pub fn search_128_256(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
    let data = req.data_bytes.as_u64x2_le()?;
    let expected = req.expected_bytes.as_u64x2_le()?;

    match req.operation {
        Operation::Encrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u64x4_le(),
            encrypt_block_128_256,
        ),
        Operation::Decrypt => run_search(
            req,
            data,
            expected,
            |k| k.as_u64x4_le(),
            decrypt_block_128_256,
        ),
    }
}
