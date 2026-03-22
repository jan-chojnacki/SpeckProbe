use crate::error::SPECKError;
use crate::error::SPECKError::{InvalidIvLength, InvalidKeyLength, IvMissing};
use crate::types::{CipherOperationMode, SpeckVersion};

mod core;
mod mode;

pub struct SPECK {
    speck_version: SpeckVersion,
    cipher_operation_mode: CipherOperationMode,
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl SPECK {
    pub fn new(
        speck_version: SpeckVersion,
        cipher_operation_mode: CipherOperationMode,
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Self, SPECKError> {
        let key_length = key.len();
        let expected_key_length = speck_version.key_size_bytes();

        if key_length != expected_key_length {
            return Err(InvalidKeyLength {
                expected: expected_key_length,
                got: key_length,
            });
        }

        let mut key_buf = vec![0u8; key_length];
        key_buf.copy_from_slice(key);

        let mut iv_buf = Vec::new();
        if cipher_operation_mode == CipherOperationMode::CBC {
            match iv {
                None => {
                    return Err(IvMissing {
                        cipher_operation_mode,
                    });
                }
                Some(iv) => {
                    let iv_length = iv.len();
                    let expected_iv_length = speck_version.block_size_bytes();

                    if iv_length != expected_iv_length {
                        return Err(InvalidIvLength {
                            expected: expected_iv_length,
                            got: iv_length,
                        });
                    }

                    iv_buf = vec![0u8; iv_length];
                    iv_buf.copy_from_slice(iv);
                }
            }
        }

        Ok(Self {
            speck_version,
            cipher_operation_mode,
            key: key_buf,
            iv: iv_buf,
        })
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        match self.cipher_operation_mode {
            CipherOperationMode::ECB => self.encrypt_ecb(data),
            CipherOperationMode::CBC => self.encrypt_cbc(data),
        }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        match self.cipher_operation_mode {
            CipherOperationMode::ECB => self.decrypt_ecb(data),
            CipherOperationMode::CBC => self.decrypt_cbc(data),
        }
    }
}
