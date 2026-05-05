use crate::cipher::speck::SPECK;
use crate::probe::ProbeError;
use crate::search::executor::CipherMode;
use crate::speck::SpeckVersion;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

/// Encrypts `data` and prints plaintext, ciphertext, and (for CBC) IV as base64.
/// All values are encoded in the same format accepted by the search config.
pub fn handle_encrypt(
    speck_version: SpeckVersion,
    cipher_mode: CipherMode,
    key: Vec<u8>,
    iv: Option<Vec<u8>>,
    data: String,
) -> Result<(), ProbeError> {
    let speck = SPECK::new(
        speck_version.into(),
        cipher_mode.into(),
        &key,
        iv.as_deref(),
    )?;

    let ciphertext = speck.encrypt(data.as_bytes())?;
    let plaintext = speck.decrypt(&ciphertext)?;

    let version: crate::speck::SpeckVersion = speck_version.into();
    println!("plaintext:  {}", to_base64(&plaintext, version));
    println!("ciphertext: {}", to_base64(&ciphertext, version));
    if cipher_mode == CipherMode::Cbc {
        println!("iv:         {}", to_base64(&speck.iv, version));
    }
    Ok(())
}

fn to_base64(data: &[u8], version: crate::speck::SpeckVersion) -> String {
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
