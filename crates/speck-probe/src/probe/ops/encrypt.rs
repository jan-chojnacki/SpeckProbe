use crate::cipher::speck::SPECK;
use crate::probe::config::enums::SpeckVersion;
use crate::probe::error::ProbeError;
use crate::runtime::api::CipherMode;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

/// Encrypts `data` with the given SPECK variant and mode.
///
/// Returns `(plaintext_b64, ciphertext_b64)` where both strings encode the
/// word-aligned byte representation using standard base64.
///
/// # Example
/// ```
/// use probe::config::enums::{CipherMode, SpeckVersion};
/// let (pt, ct) = probe::ops::encrypt::execute(
///     SpeckVersion::Speck32_64,
///     CipherMode::Ecb,
///     vec![0u8; 8],
///     None,
///     "Hi world".into(),
/// ).unwrap();
/// assert_ne!(pt, ct);
/// ```
pub fn execute(
    speck_version: SpeckVersion,
    cipher_mode: CipherMode,
    key: Vec<u8>,
    iv: Option<Vec<u8>>,
    data: String,
) -> Result<(String, String), ProbeError> {
    let speck = SPECK::new(
        speck_version.into(),
        cipher_mode.into(),
        &key,
        iv.as_deref(),
    )?;

    let ciphertext = speck.encrypt(data.as_bytes())?;
    let plaintext = speck.decrypt(&ciphertext)?;

    let version: speck::SpeckVersion = speck_version.into();
    Ok((
        to_base64(&plaintext, version),
        to_base64(&ciphertext, version),
    ))
}

fn to_base64(data: &[u8], version: speck::SpeckVersion) -> String {
    let bytes: Vec<u8> = data
        .chunks(version.word_size_bytes())
        .flat_map(|chunk| {
            let mut buf = [0u8; 8];
            buf[..version.word_size_bytes()].copy_from_slice(chunk);
            buf
        })
        .collect();
    STANDARD.encode(&bytes)
}
