use crate::cipher::SPECK;

impl SPECK {
    pub(in crate::cipher) fn add_pkcs7_padding(&self, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::from(data);

        let expected_length = self.speck_version.block_size_bytes();
        let length = out.len();

        let remainder = length % expected_length;
        let padding_size = if remainder == 0 {
            expected_length
        } else {
            expected_length - remainder
        };

        for _ in 0..padding_size {
            out.push(padding_size as u8)
        }

        out
    }

    pub(in crate::cipher) fn strip_pkcs7_padding(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let pad = *data.last().unwrap() as usize;
        if pad == 0 || pad > self.speck_version.block_size_bytes() || pad > data.len() {
            return data.to_vec();
        }

        let tail = &data[data.len() - pad..];
        if tail.iter().all(|b| *b as usize == pad) {
            data[..data.len() - pad].to_vec()
        } else {
            data.to_vec()
        }
    }
}
