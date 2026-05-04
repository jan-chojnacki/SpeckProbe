use crate::cipher::error::SPECKError;
use crate::cipher::error::SPECKError::{InvalidIvLength, InvalidKeyLength, IvMissing};
use crate::search::executor::CipherMode;
use crate::speck::SpeckVersion;

/// SPECK block cipher wrapper supporting ECB and CBC operation modes.
pub struct SPECK {
    pub speck_version: SpeckVersion,
    pub mode: CipherMode,
    pub key: Vec<u8>,
    pub iv: Vec<u8>,
}

impl SPECK {
    /// Creates a new SPECK cipher instance.
    ///
    /// Returns an error if the key length, IV length, or missing IV is incorrect
    /// for the chosen `speck_version` and `mode`.
    pub fn new(
        speck_version: SpeckVersion,
        mode: CipherMode,
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

        let mut iv_buf = Vec::new();
        if mode == CipherMode::Cbc {
            match iv {
                None => {
                    return Err(IvMissing { mode });
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

                    iv_buf = iv.to_vec();
                }
            }
        }

        Ok(Self {
            speck_version,
            mode,
            key: key.to_vec(),
            iv: iv_buf,
        })
    }

    /// Encrypts `data` using the configured cipher version and mode.
    /// Input is PKCS#7-padded to the block boundary before encryption.
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        match self.mode {
            CipherMode::Ecb => self.encrypt_ecb(data),
            CipherMode::Cbc => self.encrypt_cbc(data),
        }
    }

    /// Decrypts `data` using the configured cipher version and mode.
    /// Returns an error if `data` is not a multiple of the block size.
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        match self.mode {
            CipherMode::Ecb => self.decrypt_ecb(data),
            CipherMode::Cbc => self.decrypt_cbc(data),
        }
    }

    /// Appends PKCS#7 padding so the data length is a multiple of the block size.
    /// Always adds at least one full block of padding (even if already aligned).
    pub fn add_pkcs7_padding(&self, data: &[u8]) -> Vec<u8> {
        let block = self.speck_version.block_size_bytes() * 2;
        let remainder = data.len() % block;
        let padding_size = if remainder == 0 {
            block
        } else {
            block - remainder
        };
        let mut out = Vec::from(data);
        out.resize(out.len() + padding_size, padding_size as u8);
        out
    }
}
