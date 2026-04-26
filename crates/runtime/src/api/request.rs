use crate::TaskDone;
use crossbeam::channel::Sender;
use speck::SpeckVersion;

pub type DispatchOutput = (Vec<Vec<u8>>, Option<Vec<u8>>);

#[derive(Debug, Clone)]
pub struct RuntimeRequest {
    pub cipher_config: CipherConfig,
    pub runtime_config: RuntimeConfig,
    pub search_space: SearchSpace,
    pub internal_config: InternalConfig,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub suffix_bytes_size: usize,
    pub num_threads: usize,
    pub backend_hint: BackendHint,
}

#[derive(Debug, Clone)]
pub struct InternalConfig {
    pub cli_tx: Option<Sender<TaskDone>>,
}

#[derive(Debug, Clone)]
pub struct CipherConfig {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
    pub cipher_function: CipherFunction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
pub enum CipherFunction {
    Encrypt,
    Decrypt,
    EncryptInflight,
}

#[derive(Debug, Clone)]
pub struct SearchSpace {
    pub start: Vec<u8>,
    pub end: Vec<u8>,
    pub data: Vec<[u64; 2]>,
    pub expected: Vec<[u64; 2]>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
pub enum CipherMode {
    Ecb,
    Cbc,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum::Display)]
pub enum BackendHint {
    Auto,
    Scalar,
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    Sse2,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    Avx2,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
    Avx512,
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    Neon,
}
