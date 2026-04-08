use clap::Parser;
use std::path::PathBuf;

const CRITERION_PATH: &str = "./target/criterion/";
const OUTPUT_PATH: &str = "output.csv";

/// Extract Criterion results from raw CSV files
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Ścieżka do katalogu z wynikami Criterion (szuka plików raw.csv w podkatalogach new)
    #[arg(short, long, default_value = CRITERION_PATH)]
    pub(crate) criterion_path: PathBuf,

    /// Ścieżka pliku wyjściowego CSV z połączonymi rekordami
    #[arg(short, long, default_value = OUTPUT_PATH)]
    pub(crate) output_path: PathBuf,
}
