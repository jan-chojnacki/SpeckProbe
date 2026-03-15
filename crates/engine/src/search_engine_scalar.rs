use crate::key::Key;
use crate::key_iterator::KeyIterator;
use crate::search_range_request::SearchRangeRequest;
use crate::speck_version::SpeckVersion;
use crate::SearchEngineBackend;
use speck::{
    decrypt_block_128_128, decrypt_block_128_192, decrypt_block_128_256, decrypt_block_32_64,
    decrypt_block_48_72, decrypt_block_48_96, decrypt_block_64_128, decrypt_block_64_96,
    decrypt_block_96_144, decrypt_block_96_96, encrypt_block_128_128, encrypt_block_128_192,
    encrypt_block_128_256, encrypt_block_32_64, encrypt_block_48_72, encrypt_block_48_96,
    encrypt_block_64_128, encrypt_block_64_96, encrypt_block_96_144, encrypt_block_96_96,
};

pub struct SearchEngineScalar {}

impl SearchEngineBackend for SearchEngineScalar {
    fn search_range_encrypt(search_range_request: SearchRangeRequest) -> Option<Vec<Key>> {
        let speck_version = search_range_request.speck_version;
        let start_key = search_range_request.start_key;
        let key_count = search_range_request.key_count;
        let prefix = search_range_request.prefix;

        let mut iterator = KeyIterator::new(start_key, key_count, &prefix, &speck_version).unwrap();
        let mut key = iterator.new_key();

        let mut results: Vec<Key> = Vec::new();

        match speck_version {
            SpeckVersion::Speck32_64 => {
                let data = search_range_request.data_bytes.as_u16x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u16x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_32_64(data, key.as_u16x4_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck48_72 => {
                let data = search_range_request.data_bytes.as_u24x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u24x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_48_72(data, key.as_u24x3_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck48_96 => {
                let data = search_range_request.data_bytes.as_u24x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u24x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_48_96(data, key.as_u24x4_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck64_96 => {
                let data = search_range_request.data_bytes.as_u32x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u32x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_64_96(data, key.as_u32x3_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck64_128 => {
                let data = search_range_request.data_bytes.as_u32x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u32x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_64_128(data, key.as_u32x4_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck96_96 => {
                let data = search_range_request.data_bytes.as_u48x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u48x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_96_96(data, key.as_u48x2_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck96_144 => {
                let data = search_range_request.data_bytes.as_u48x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u48x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_96_144(data, key.as_u48x3_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck128_128 => {
                let data = search_range_request.data_bytes.as_u64x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u64x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_128_128(data, key.as_u64x2_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck128_192 => {
                let data = search_range_request.data_bytes.as_u64x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u64x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_128_192(data, key.as_u64x3_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck128_256 => {
                let data = search_range_request.data_bytes.as_u64x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u64x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = encrypt_block_128_256(data, key.as_u64x4_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
        }

        match results.is_empty() {
            true => None,
            false => Some(results),
        }
    }

    fn search_range_decrypt(search_range_request: SearchRangeRequest) -> Option<Vec<Key>> {
        let speck_version = search_range_request.speck_version;
        let start_key = search_range_request.start_key;
        let key_count = search_range_request.key_count;
        let prefix = search_range_request.prefix;

        let mut iterator = KeyIterator::new(start_key, key_count, &prefix, &speck_version).unwrap();
        let mut key = iterator.new_key();

        let mut results: Vec<Key> = Vec::new();

        match speck_version {
            SpeckVersion::Speck32_64 => {
                let data = search_range_request.data_bytes.as_u16x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u16x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_32_64(data, key.as_u16x4_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck48_72 => {
                let data = search_range_request.data_bytes.as_u24x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u24x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_48_72(data, key.as_u24x3_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck48_96 => {
                let data = search_range_request.data_bytes.as_u24x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u24x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_48_96(data, key.as_u24x4_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck64_96 => {
                let data = search_range_request.data_bytes.as_u32x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u32x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_64_96(data, key.as_u32x3_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck64_128 => {
                let data = search_range_request.data_bytes.as_u32x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u32x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_64_128(data, key.as_u32x4_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck96_96 => {
                let data = search_range_request.data_bytes.as_u48x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u48x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_96_96(data, key.as_u48x2_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck96_144 => {
                let data = search_range_request.data_bytes.as_u48x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u48x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_96_144(data, key.as_u48x3_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck128_128 => {
                let data = search_range_request.data_bytes.as_u64x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u64x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_128_128(data, key.as_u64x2_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck128_192 => {
                let data = search_range_request.data_bytes.as_u64x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u64x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_128_192(data, key.as_u64x3_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
            SpeckVersion::Speck128_256 => {
                let data = search_range_request.data_bytes.as_u64x2_le().unwrap();
                let expected = search_range_request.expected_bytes.as_u64x2_le().unwrap();

                while iterator.next_into(&mut key).is_some() {
                    let result = decrypt_block_128_256(data, key.as_u64x4_le());
                    if result == expected {
                        results.push(key);
                    }
                }
            }
        }

        match results.is_empty() {
            true => None,
            false => Some(results),
        }
    }
}
