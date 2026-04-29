use crate::cipher::speck::SPECK;
use std::array::from_fn;

impl SPECK {
    pub(super) fn speck_ecb_encrypt<W: Copy, const WORD_BYTES: usize, const KEY_WORDS: usize>(
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

        for block in padded.chunks_exact(2 * WORD_BYTES) {
            let plaintext_block: [W; 2] = from_fn(|i| {
                let s = i * WORD_BYTES;
                read_word(&block[s..s + WORD_BYTES])
            });
            let ciphertext_block = block_cipher(plaintext_block, key_words);
            write_word(ciphertext_block[0], &mut output);
            write_word(ciphertext_block[1], &mut output);
        }

        output
    }

    pub(super) fn speck_ecb_decrypt<W: Copy, const WORD_BYTES: usize, const KEY_WORDS: usize>(
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

        for block in data.chunks_exact(2 * WORD_BYTES) {
            let ciphertext_block: [W; 2] = from_fn(|i| {
                let s = i * WORD_BYTES;
                read_word(&block[s..s + WORD_BYTES])
            });
            let plaintext_block = block_cipher(ciphertext_block, key_words);
            write_word(plaintext_block[0], &mut output);
            write_word(plaintext_block[1], &mut output);
        }

        output
    }
}
