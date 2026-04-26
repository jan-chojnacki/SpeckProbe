use clap::Parser;
use cli::application::benchmark::{BenchmarkTarget, BenchmarkTargets};
use cli::application::{benchmark, generate_config, search};
use cli::domain::config::{BackendHint, CipherFunction, CipherMode, SpeckVersion};
use cli::presentation::args::{Args, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Search {
            config_path,
            spurious,
        } => search::execute(config_path, spurious)?,
        Commands::SampleConfig { config_path, force } => {
            generate_config::execute(config_path, force)?
        }
        Commands::Benchmark { bits } => {
            let targets: BenchmarkTargets = vec![
                BenchmarkTarget {
                    cipher_mode: CipherMode::Ecb,
                    speck_version: SpeckVersion::Speck32_64,
                    cipher_function: CipherFunction::EncryptInflight,
                    backend_hint: BackendHint::Avx512,
                    suffix_bytes: 2,
                },
                BenchmarkTarget {
                    cipher_mode: CipherMode::Ecb,
                    speck_version: SpeckVersion::Speck32_64,
                    cipher_function: CipherFunction::EncryptInflight,
                    backend_hint: BackendHint::Avx2,
                    suffix_bytes: 2,
                },
                BenchmarkTarget {
                    cipher_mode: CipherMode::Ecb,
                    speck_version: SpeckVersion::Speck32_64,
                    cipher_function: CipherFunction::EncryptInflight,
                    backend_hint: BackendHint::Sse2,
                    suffix_bytes: 2,
                },
                BenchmarkTarget {
                    cipher_mode: CipherMode::Ecb,
                    speck_version: SpeckVersion::Speck32_64,
                    cipher_function: CipherFunction::EncryptInflight,
                    backend_hint: BackendHint::Scalar,
                    suffix_bytes: 2,
                },
            ];
            benchmark::execute(targets, bits)?
        }
    }

    Ok(())
}
