use crate::cipher::SPECK;
use crate::codec::{
    read_u16_le, read_u24_le, read_u32_le, read_u48_le, read_u64_le, write_u16_le, write_u24_le,
    write_u32_le, write_u48_le, write_u64_le,
};
use crate::error::SPECKError;
use speck::{SpeckVersion, U24, U48};
use speck::{
    decrypt_block_32_64, decrypt_block_48_72, decrypt_block_48_96, decrypt_block_64_96,
    decrypt_block_64_128, decrypt_block_96_96, decrypt_block_96_144, decrypt_block_128_128,
    decrypt_block_128_192, decrypt_block_128_256, encrypt_block_32_64, encrypt_block_48_72,
    encrypt_block_48_96, encrypt_block_64_96, encrypt_block_64_128, encrypt_block_96_96,
    encrypt_block_96_144, encrypt_block_128_128, encrypt_block_128_192, encrypt_block_128_256,
};
use std::array::from_fn;
use std::ops::BitXor;

impl SPECK {
    fn speck_cbc_encrypt<
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

    fn speck_cbc_decrypt<
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

        self.strip_pkcs7_padding(&output)
    }

    pub(in crate::cipher) fn encrypt_cbc(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        let out = match self.speck_version {
            SpeckVersion::Speck32_64 => self.speck_cbc_encrypt::<u16, 2, 4>(
                data,
                encrypt_block_32_64,
                read_u16_le,
                write_u16_le,
            ),
            SpeckVersion::Speck48_72 => self.speck_cbc_encrypt::<U24, 3, 3>(
                data,
                encrypt_block_48_72,
                read_u24_le,
                write_u24_le,
            ),
            SpeckVersion::Speck48_96 => self.speck_cbc_encrypt::<U24, 3, 4>(
                data,
                encrypt_block_48_96,
                read_u24_le,
                write_u24_le,
            ),
            SpeckVersion::Speck64_96 => self.speck_cbc_encrypt::<u32, 4, 3>(
                data,
                encrypt_block_64_96,
                read_u32_le,
                write_u32_le,
            ),
            SpeckVersion::Speck64_128 => self.speck_cbc_encrypt::<u32, 4, 4>(
                data,
                encrypt_block_64_128,
                read_u32_le,
                write_u32_le,
            ),
            SpeckVersion::Speck96_96 => self.speck_cbc_encrypt::<U48, 6, 2>(
                data,
                encrypt_block_96_96,
                read_u48_le,
                write_u48_le,
            ),
            SpeckVersion::Speck96_144 => self.speck_cbc_encrypt::<U48, 6, 3>(
                data,
                encrypt_block_96_144,
                read_u48_le,
                write_u48_le,
            ),
            SpeckVersion::Speck128_128 => self.speck_cbc_encrypt::<u64, 8, 2>(
                data,
                encrypt_block_128_128,
                read_u64_le,
                write_u64_le,
            ),
            SpeckVersion::Speck128_192 => self.speck_cbc_encrypt::<u64, 8, 3>(
                data,
                encrypt_block_128_192,
                read_u64_le,
                write_u64_le,
            ),
            SpeckVersion::Speck128_256 => self.speck_cbc_encrypt::<u64, 8, 4>(
                data,
                encrypt_block_128_256,
                read_u64_le,
                write_u64_le,
            ),
        };

        Ok(out)
    }

    pub(in crate::cipher) fn decrypt_cbc(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        let block_size = self.speck_version.block_size_bytes();
        if data.len() % block_size != 0 {
            return Err(SPECKError::InvalidDataLength {
                expected_multiple: block_size,
                got: data.len(),
            });
        }

        let out = match self.speck_version {
            SpeckVersion::Speck32_64 => self.speck_cbc_decrypt::<u16, 2, 4>(
                data,
                decrypt_block_32_64,
                read_u16_le,
                write_u16_le,
            ),
            SpeckVersion::Speck48_72 => self.speck_cbc_decrypt::<u32, 3, 3>(
                data,
                decrypt_block_48_72,
                read_u24_le,
                write_u24_le,
            ),
            SpeckVersion::Speck48_96 => self.speck_cbc_decrypt::<u32, 3, 4>(
                data,
                decrypt_block_48_96,
                read_u24_le,
                write_u24_le,
            ),
            SpeckVersion::Speck64_96 => self.speck_cbc_decrypt::<u32, 4, 3>(
                data,
                decrypt_block_64_96,
                read_u32_le,
                write_u32_le,
            ),
            SpeckVersion::Speck64_128 => self.speck_cbc_decrypt::<u32, 4, 4>(
                data,
                decrypt_block_64_128,
                read_u32_le,
                write_u32_le,
            ),
            SpeckVersion::Speck96_96 => self.speck_cbc_decrypt::<u64, 6, 2>(
                data,
                decrypt_block_96_96,
                read_u48_le,
                write_u48_le,
            ),
            SpeckVersion::Speck96_144 => self.speck_cbc_decrypt::<u64, 6, 3>(
                data,
                decrypt_block_96_144,
                read_u48_le,
                write_u48_le,
            ),
            SpeckVersion::Speck128_128 => self.speck_cbc_decrypt::<u64, 8, 2>(
                data,
                decrypt_block_128_128,
                read_u64_le,
                write_u64_le,
            ),
            SpeckVersion::Speck128_192 => self.speck_cbc_decrypt::<u64, 8, 3>(
                data,
                decrypt_block_128_192,
                read_u64_le,
                write_u64_le,
            ),
            SpeckVersion::Speck128_256 => self.speck_cbc_decrypt::<u64, 8, 4>(
                data,
                decrypt_block_128_256,
                read_u64_le,
                write_u64_le,
            ),
        };

        Ok(out)
    }
}
