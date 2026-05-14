use crate::cipher::speck::SPECK;
use std::array::from_fn;
use std::ops::BitXor;

impl SPECK {
    pub(super) fn speck_cbc_encrypt<
        W: Copy + BitXor<Output = W>,
        const WORD_BYTES: usize,
        const KEY_WORDS: usize,
    >(
        &self,
        data: &[u8],
        block_cipher: fn([W; 2], [W; KEY_WORDS]) -> [W; 2],
        read_word: fn(&[u8]) -> W,
        write_word: fn(W, &mut Vec<u8>),
    ) -> Vec<u8> {
        let padded = self.add_pkcs7_padding(data);
        let mut output = Vec::with_capacity(padded.len());

        let key_words: [W; KEY_WORDS] = from_fn(|i| {
            let s = i * WORD_BYTES;
            read_word(&self.key[s..s + WORD_BYTES])
        });

        let mut previous_cipher_block: [W; 2] = from_fn(|i| {
            let s = i * WORD_BYTES;
            read_word(&self.iv[s..s + WORD_BYTES])
        });

        for block in padded.chunks_exact(2 * WORD_BYTES) {
            let plaintext_block: [W; 2] = from_fn(|i| {
                let s = i * WORD_BYTES;
                read_word(&block[s..s + WORD_BYTES])
            });
            let plaintext_xor_prev = [
                plaintext_block[0] ^ previous_cipher_block[0],
                plaintext_block[1] ^ previous_cipher_block[1],
            ];
            let ciphertext_block = block_cipher(plaintext_xor_prev, key_words);
            previous_cipher_block = ciphertext_block;
            write_word(ciphertext_block[0], &mut output);
            write_word(ciphertext_block[1], &mut output);
        }

        output
    }

    pub(super) fn speck_cbc_decrypt<
        W: Copy + BitXor<Output = W>,
        const WORD_BYTES: usize,
        const KEY_WORDS: usize,
    >(
        &self,
        data: &[u8],
        block_cipher: fn([W; 2], [W; KEY_WORDS]) -> [W; 2],
        read_word: fn(&[u8]) -> W,
        write_word: fn(W, &mut Vec<u8>),
    ) -> Vec<u8> {
        let mut output = Vec::with_capacity(data.len());

        let key_words: [W; KEY_WORDS] = from_fn(|i| {
            let s = i * WORD_BYTES;
            read_word(&self.key[s..s + WORD_BYTES])
        });

        let mut previous_cipher_block: [W; 2] = from_fn(|i| {
            let s = i * WORD_BYTES;
            read_word(&self.iv[s..s + WORD_BYTES])
        });

        for block in data.chunks_exact(2 * WORD_BYTES) {
            let ciphertext_block: [W; 2] = from_fn(|i| {
                let s = i * WORD_BYTES;
                read_word(&block[s..s + WORD_BYTES])
            });
            let decrypted_block = block_cipher(ciphertext_block, key_words);
            let plaintext_block = [
                decrypted_block[0] ^ previous_cipher_block[0],
                decrypted_block[1] ^ previous_cipher_block[1],
            ];
            previous_cipher_block = ciphertext_block;
            write_word(plaintext_block[0], &mut output);
            write_word(plaintext_block[1], &mut output);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::cipher::error::SPECKError;
    use crate::cipher::speck::SPECK;
    use crate::search::executor::CipherMode;
    use crate::speck::SpeckVersion;
    use rstest::rstest;

    fn make_cbc(version: SpeckVersion, key: &[u8], iv: &[u8]) -> SPECK {
        SPECK::new(version, CipherMode::Cbc, key, Some(iv)).unwrap()
    }

    #[rstest]
    #[case(SpeckVersion::Speck32_64,   vec![0x18u8,0x19,0x10,0x11,0x08,0x09,0x00,0x01],                                                                                                                         vec![0u8;4])]
    #[case(SpeckVersion::Speck48_72,   vec![0x10u8,0x11,0x12,0x08,0x09,0x0a,0x00,0x01,0x02],                                                                                                                    vec![0u8;6])]
    #[case(SpeckVersion::Speck48_96,   vec![0x18u8,0x19,0x1a,0x10,0x11,0x12,0x08,0x09,0x0a,0x00,0x01,0x02],                                                                                                     vec![0u8;6])]
    #[case(SpeckVersion::Speck64_96,   vec![0x10u8,0x11,0x12,0x13,0x08,0x09,0x0a,0x0b,0x00,0x01,0x02,0x03],                                                                                                     vec![0u8;8])]
    #[case(SpeckVersion::Speck64_128,  vec![0x18u8,0x19,0x1a,0x1b,0x10,0x11,0x12,0x13,0x08,0x09,0x0a,0x0b,0x00,0x01,0x02,0x03],                                                                                 vec![0u8;8])]
    #[case(SpeckVersion::Speck96_96,   vec![0x08u8,0x09,0x0a,0x0b,0x0c,0x0d,0x00,0x01,0x02,0x03,0x04,0x05],                                                                                                     vec![0u8;12])]
    #[case(SpeckVersion::Speck96_144,  vec![0x10u8,0x11,0x12,0x13,0x14,0x15,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x00,0x01,0x02,0x03,0x04,0x05],                                                                       vec![0u8;12])]
    #[case(SpeckVersion::Speck128_128, vec![0x08u8,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f,0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07],                                                                                 vec![0u8;16])]
    #[case(SpeckVersion::Speck128_192, vec![0x10u8,0x11,0x12,0x13,0x14,0x15,0x16,0x17,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f,0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07],                                         vec![0u8;16])]
    #[case(SpeckVersion::Speck128_256, vec![0x18u8,0x19,0x1a,0x1b,0x1c,0x1d,0x1e,0x1f,0x10,0x11,0x12,0x13,0x14,0x15,0x16,0x17,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f,0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07], vec![0u8;16])]
    fn roundtrip(#[case] version: SpeckVersion, #[case] key: Vec<u8>, #[case] iv: Vec<u8>) {
        let speck = make_cbc(version, &key, &iv);
        let pt = b"hello world";
        let ct = speck.encrypt(pt).unwrap();
        let recovered = speck.decrypt(&ct).unwrap();
        assert_eq!(recovered, speck.add_pkcs7_padding(pt));
    }

    #[test]
    fn decrypt_unaligned_length_returns_error() {
        let err = make_cbc(SpeckVersion::Speck32_64, &[0u8; 8], &[0u8; 4])
            .decrypt(&[0u8; 3])
            .unwrap_err();
        assert_eq!(
            err,
            SPECKError::InvalidDataLength {
                expected_multiple: 4,
                got: 3
            }
        );
    }
}
