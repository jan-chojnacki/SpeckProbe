use crate::probe::ProbeError;
use crate::probe::config::enums::{CipherMode, SpeckVersion};
use crate::probe::ops::encrypt::execute as run_encrypt;

/// Encrypts `data` and prints both the resulting plaintext and ciphertext as base64.
pub fn execute(
    speck_version: SpeckVersion,
    cipher_mode: CipherMode,
    key: Vec<u8>,
    iv: Option<Vec<u8>>,
    data: String,
) -> Result<(), ProbeError> {
    let (plaintext, ciphertext) = run_encrypt(speck_version, cipher_mode, key, iv, data)?;
    println!("plaintext:  {}", plaintext);
    println!("ciphertext: {}", ciphertext);
    Ok(())
}
