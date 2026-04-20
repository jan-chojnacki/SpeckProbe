use super::{BackendHint, CipherFunction, CipherMode, Config, SpeckVersion};

pub fn generate_sample() -> Config {
    Config {
        cipher_mode: CipherMode::Ecb,
        speck_version: SpeckVersion::Speck32_64,
        cipher_function: CipherFunction::EncryptInflight,
        suffix_bytes_size: 3,
        num_threads: num_cpus::get(),
        backend_hint: BackendHint::Auto,
        start: vec![0; 5],
        end: vec![255, 15, 0, 0, 0],
        data: vec![[0, 0], [1, 1]],
        expected: vec![[0, 0], [1, 1]],
    }
}
