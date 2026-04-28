use crate::application::error::ApplicationError;
use crate::domain::config::enums::{CipherMode, SpeckVersion};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use utils::SPECK;

pub fn execute(
    speck_version: SpeckVersion,
    cipher_mode: CipherMode,
    key: Vec<u8>,
    iv: Option<Vec<u8>>,
    data: String,
) -> Result<(), ApplicationError> {
    let speck = SPECK::new(
        speck_version.into(),
        cipher_mode.into(),
        &key,
        iv.as_deref(),
    )?;

    let text = speck.encrypt(data.as_bytes())?;
    let known = speck.decrypt(&text)?;
    println!("plaintext:  {}", to_base64(&known, speck_version.into()));
    println!("ciphertext: {}", to_base64(&text, speck_version.into()));

    Ok(())
}

fn to_base64(text: &[u8], speck_version: speck::SpeckVersion) -> String {
    let bytes: Vec<u64> = text
        .chunks(speck_version.block_size_bytes())
        .map(|x| {
            let mut buff = [0u8; 8];
            buff[..speck_version.block_size_bytes()].copy_from_slice(x);
            u64::from_le_bytes(buff)
        })
        .collect();

    let bytes: Vec<u8> = bytes.iter().flat_map(|x1| x1.to_le_bytes()).collect();

    STANDARD.encode(&bytes)
}
