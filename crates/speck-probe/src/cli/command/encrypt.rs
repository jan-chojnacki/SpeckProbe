use crate::cipher::speck::SPECK;
use crate::speck::SpeckVersion;
use crate::probe::ProbeError;
use crate::search::executor::CipherMode;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

/// Encrypts `data` and prints both the resulting plaintext and ciphertext as base64.
pub fn handle_encrypt(
    speck_version: SpeckVersion,
    cipher_mode: CipherMode,
    key: Vec<u8>,
    iv: Option<Vec<u8>>,
    data: String,
) -> Result<(), ProbeError> {
    let (plaintext, ciphertext) = execute(speck_version, cipher_mode, key, iv, data)?;
    println!("plaintext:  {}", plaintext);
    println!("ciphertext: {}", ciphertext);
    Ok(())
}

fn execute(
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

    let version: crate::speck::SpeckVersion = speck_version.into();
    Ok((
        to_base64(&plaintext, version),
        to_base64(&ciphertext, version),
    ))
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
