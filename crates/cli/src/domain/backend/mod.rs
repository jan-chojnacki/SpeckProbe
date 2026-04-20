#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveBackend {
    Avx512,
    Avx2,
    Sse2,
    Neon,
    Scalar,
}

impl std::fmt::Display for ActiveBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveBackend::Avx512 => write!(f, "AVX-512"),
            ActiveBackend::Avx2 => write!(f, "AVX2"),
            ActiveBackend::Sse2 => write!(f, "SSE2"),
            ActiveBackend::Neon => write!(f, "NEON"),
            ActiveBackend::Scalar => write!(f, "Scalar"),
        }
    }
}

pub fn detect_auto_backend() -> ActiveBackend {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512bw") {
            return ActiveBackend::Avx512;
        }
        if is_x86_feature_detected!("avx2") {
            return ActiveBackend::Avx2;
        }
        if is_x86_feature_detected!("sse2") {
            return ActiveBackend::Sse2;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("neon") {
            return ActiveBackend::Neon;
        }
    }
    ActiveBackend::Scalar
}
