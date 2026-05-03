pub mod cbc;
pub mod ecb;

use crate::cipher::error::SPECKError;
use crate::cipher::speck::SPECK;
use crate::speck::SpeckVersion;

/// Dispatches a generic cipher method call to the correct const-generic instantiation.
///
/// Usage: `speck_dispatch!(encrypt: self.method(data))` or `speck_dispatch!(decrypt: self.method(data))`.
macro_rules! speck_dispatch {
    (encrypt: $self:ident . $method:ident ( $data:expr )) => {
        match $self.speck_version {
            SpeckVersion::Speck32_64 => $self.$method::<u16, 2, 4>(
                $data,
                $crate::speck::scalar_encrypt_block_32_64,
                $crate::cipher::codec::read_u16_le,
                $crate::cipher::codec::write_u16_le,
            ),
            SpeckVersion::Speck48_72 => $self.$method::<u32, 3, 3>(
                $data,
                $crate::speck::scalar_encrypt_block_48_72,
                $crate::cipher::codec::read_u24_le,
                $crate::cipher::codec::write_u24_le,
            ),
            SpeckVersion::Speck48_96 => $self.$method::<u32, 3, 4>(
                $data,
                $crate::speck::scalar_encrypt_block_48_96,
                $crate::cipher::codec::read_u24_le,
                $crate::cipher::codec::write_u24_le,
            ),
            SpeckVersion::Speck64_96 => $self.$method::<u32, 4, 3>(
                $data,
                $crate::speck::scalar_encrypt_block_64_96,
                $crate::cipher::codec::read_u32_le,
                $crate::cipher::codec::write_u32_le,
            ),
            SpeckVersion::Speck64_128 => $self.$method::<u32, 4, 4>(
                $data,
                $crate::speck::scalar_encrypt_block_64_128,
                $crate::cipher::codec::read_u32_le,
                $crate::cipher::codec::write_u32_le,
            ),
            SpeckVersion::Speck96_96 => $self.$method::<u64, 6, 2>(
                $data,
                $crate::speck::scalar_encrypt_block_96_96,
                $crate::cipher::codec::read_u48_le,
                $crate::cipher::codec::write_u48_le,
            ),
            SpeckVersion::Speck96_144 => $self.$method::<u64, 6, 3>(
                $data,
                $crate::speck::scalar_encrypt_block_96_144,
                $crate::cipher::codec::read_u48_le,
                $crate::cipher::codec::write_u48_le,
            ),
            SpeckVersion::Speck128_128 => $self.$method::<u64, 8, 2>(
                $data,
                $crate::speck::scalar_encrypt_block_128_128,
                $crate::cipher::codec::read_u64_le,
                $crate::cipher::codec::write_u64_le,
            ),
            SpeckVersion::Speck128_192 => $self.$method::<u64, 8, 3>(
                $data,
                $crate::speck::scalar_encrypt_block_128_192,
                $crate::cipher::codec::read_u64_le,
                $crate::cipher::codec::write_u64_le,
            ),
            SpeckVersion::Speck128_256 => $self.$method::<u64, 8, 4>(
                $data,
                $crate::speck::scalar_encrypt_block_128_256,
                $crate::cipher::codec::read_u64_le,
                $crate::cipher::codec::write_u64_le,
            ),
        }
    };
    (decrypt: $self:ident . $method:ident ( $data:expr )) => {
        match $self.speck_version {
            SpeckVersion::Speck32_64 => $self.$method::<u16, 2, 4>(
                $data,
                $crate::speck::scalar_decrypt_block_32_64,
                $crate::cipher::codec::read_u16_le,
                $crate::cipher::codec::write_u16_le,
            ),
            SpeckVersion::Speck48_72 => $self.$method::<u32, 3, 3>(
                $data,
                $crate::speck::scalar_decrypt_block_48_72,
                $crate::cipher::codec::read_u24_le,
                $crate::cipher::codec::write_u24_le,
            ),
            SpeckVersion::Speck48_96 => $self.$method::<u32, 3, 4>(
                $data,
                $crate::speck::scalar_decrypt_block_48_96,
                $crate::cipher::codec::read_u24_le,
                $crate::cipher::codec::write_u24_le,
            ),
            SpeckVersion::Speck64_96 => $self.$method::<u32, 4, 3>(
                $data,
                $crate::speck::scalar_decrypt_block_64_96,
                $crate::cipher::codec::read_u32_le,
                $crate::cipher::codec::write_u32_le,
            ),
            SpeckVersion::Speck64_128 => $self.$method::<u32, 4, 4>(
                $data,
                $crate::speck::scalar_decrypt_block_64_128,
                $crate::cipher::codec::read_u32_le,
                $crate::cipher::codec::write_u32_le,
            ),
            SpeckVersion::Speck96_96 => $self.$method::<u64, 6, 2>(
                $data,
                $crate::speck::scalar_decrypt_block_96_96,
                $crate::cipher::codec::read_u48_le,
                $crate::cipher::codec::write_u48_le,
            ),
            SpeckVersion::Speck96_144 => $self.$method::<u64, 6, 3>(
                $data,
                $crate::speck::scalar_decrypt_block_96_144,
                $crate::cipher::codec::read_u48_le,
                $crate::cipher::codec::write_u48_le,
            ),
            SpeckVersion::Speck128_128 => $self.$method::<u64, 8, 2>(
                $data,
                $crate::speck::scalar_decrypt_block_128_128,
                $crate::cipher::codec::read_u64_le,
                $crate::cipher::codec::write_u64_le,
            ),
            SpeckVersion::Speck128_192 => $self.$method::<u64, 8, 3>(
                $data,
                $crate::speck::scalar_decrypt_block_128_192,
                $crate::cipher::codec::read_u64_le,
                $crate::cipher::codec::write_u64_le,
            ),
            SpeckVersion::Speck128_256 => $self.$method::<u64, 8, 4>(
                $data,
                $crate::speck::scalar_decrypt_block_128_256,
                $crate::cipher::codec::read_u64_le,
                $crate::cipher::codec::write_u64_le,
            ),
        }
    };
}

impl SPECK {
    pub(in crate::cipher) fn encrypt_ecb(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        Ok(speck_dispatch!(encrypt: self.speck_ecb_encrypt(data)))
    }

    pub(in crate::cipher) fn decrypt_ecb(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        let block_size = self.speck_version.block_size_bytes();
        if !data.len().is_multiple_of(block_size) {
            return Err(SPECKError::InvalidDataLength {
                expected_multiple: block_size,
                got: data.len(),
            });
        }
        Ok(speck_dispatch!(decrypt: self.speck_ecb_decrypt(data)))
    }

    pub(in crate::cipher) fn encrypt_cbc(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        Ok(speck_dispatch!(encrypt: self.speck_cbc_encrypt(data)))
    }

    pub(in crate::cipher) fn decrypt_cbc(&self, data: &[u8]) -> Result<Vec<u8>, SPECKError> {
        let block_size = self.speck_version.block_size_bytes();
        if !data.len().is_multiple_of(block_size) {
            return Err(SPECKError::InvalidDataLength {
                expected_multiple: block_size,
                got: data.len(),
            });
        }
        Ok(speck_dispatch!(decrypt: self.speck_cbc_decrypt(data)))
    }
}
