use crate::cipher::error::SPECKError;
use crate::cipher::error::SPECKError::{InvalidIvLength, InvalidKeyLength, IvMissing};
use crate::search::executor::CipherMode;
use crate::speck::SpeckVersion;

/// SPECK block cipher wrapper supporting ECB and CBC operation modes.
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn speck32_ecb() -> SPECK {
        SPECK::new(SpeckVersion::Speck32_64, CipherMode::Ecb, &[0u8; 8], None).unwrap()
    }

    #[rstest]
    #[case(SpeckVersion::Speck32_64, CipherMode::Ecb, 8, None::<Vec<u8>>)]
    #[case(SpeckVersion::Speck128_128, CipherMode::Ecb, 16, None::<Vec<u8>>)]
    #[case(SpeckVersion::Speck32_64, CipherMode::Cbc, 8, Some(vec![0u8; 4]))]
    #[case(SpeckVersion::Speck128_128, CipherMode::Cbc, 16, Some(vec![0u8; 16]))]
    fn new_succeeds(
        #[case] version: SpeckVersion,
        #[case] mode: CipherMode,
        #[case] key_len: usize,
        #[case] iv: Option<Vec<u8>>,
    ) {
        let key = vec![0u8; key_len];
        assert!(SPECK::new(version, mode, &key, iv.as_deref()).is_ok());
    }

    #[test]
    fn new_wrong_key_length() {
        let err =
            SPECK::new(SpeckVersion::Speck32_64, CipherMode::Ecb, &[0u8; 7], None).unwrap_err();
        assert_eq!(
            err,
            InvalidKeyLength {
                expected: 8,
                got: 7
            }
        );
    }

    #[test]
    fn new_cbc_missing_iv() {
        let err =
            SPECK::new(SpeckVersion::Speck32_64, CipherMode::Cbc, &[0u8; 8], None).unwrap_err();
        assert_eq!(
            err,
            IvMissing {
                mode: CipherMode::Cbc
            }
        );
    }

    #[test]
    fn new_cbc_wrong_iv_length() {
        let err = SPECK::new(
            SpeckVersion::Speck32_64,
            CipherMode::Cbc,
            &[0u8; 8],
            Some(&[0u8; 2]),
        )
        .unwrap_err();
        assert_eq!(
            err,
            InvalidIvLength {
                expected: 4,
                got: 2
            }
        );
    }

    #[test]
    fn padding_empty_input() {
        let padded = speck32_ecb().add_pkcs7_padding(&[]);
        assert_eq!(padded, vec![0x08u8; 8]);
    }

    #[test]
    fn padding_aligned_input() {
        let padded = speck32_ecb().add_pkcs7_padding(&[0u8; 8]);
        assert_eq!(padded.len(), 16);
        assert!(padded[8..].iter().all(|&b| b == 0x08));
    }

    #[rstest]
    #[case(1usize, 7u8)]
    #[case(4usize, 4u8)]
    #[case(7usize, 1u8)]
    fn padding_partial_input(#[case] input_len: usize, #[case] pad_byte: u8) {
        let input = vec![0u8; input_len];
        let padded = speck32_ecb().add_pkcs7_padding(&input);
        assert_eq!(padded.len(), 8);
        assert!(padded[input_len..].iter().all(|&b| b == pad_byte));
    }
}
