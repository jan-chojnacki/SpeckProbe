use crate::key::Key;
use crate::key_iterator::KeyIterator;
use crate::search_range_request::{Operation, SearchRangeRequest};
use crate::speck_version::SpeckVersion;
use crate::{SearchEngineBackend, SearchEngineBackendError};
use speck::{
    decrypt_block_128_128, decrypt_block_128_192, decrypt_block_128_256, decrypt_block_32_64,
    decrypt_block_48_72, decrypt_block_48_96, decrypt_block_64_128, decrypt_block_64_96,
    decrypt_block_96_144, decrypt_block_96_96, encrypt_block_128_128, encrypt_block_128_192,
    encrypt_block_128_256, encrypt_block_32_64, encrypt_block_48_72, encrypt_block_48_96,
    encrypt_block_64_128, encrypt_block_64_96, encrypt_block_96_144, encrypt_block_96_96,
};

pub struct SearchEngineScalar {}

impl SearchEngineBackend for SearchEngineScalar {
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        search_range_impl(search_range_request)
    }

    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        search_range_impl(search_range_request)
    }
}

fn search_range_impl(
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

fn run_search<R, K, FK, FC>(
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

fn search_32_64(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_48_72(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_48_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_64_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_64_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_96_96(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_96_144(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_128_128(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_128_192(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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

fn search_128_256(req: &SearchRangeRequest) -> Result<Vec<Key>, SearchEngineBackendError> {
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
