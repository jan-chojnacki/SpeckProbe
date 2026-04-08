use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

const CRITERION_PATH: &str = "./target/criterion/";
const OUTPUT_PATH: &str = "./analysis/data/output.csv";

/// Extract Criterion results from raw CSV files
#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = None,
    infer_subcommands = true,
    arg_required_else_help = true
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Wyciąga rekordy z wyników Criterion i dopisuje je do pliku CSV
    Extract(ExtractArgs),
    /// Łączy dwa pliki CSV w jeden plik wynikowy
    Merge(MergeArgs),
}

#[derive(Debug, Args)]
pub struct ExtractArgs {
    /// Ścieżka do katalogu z wynikami Criterion (szuka plików raw.csv w podkatalogach new)
    #[arg(short, long, default_value = CRITERION_PATH)]
    pub(crate) criterion_path: PathBuf,
    /// Ścieżka pliku wyjściowego CSV z połączonymi rekordami
    #[arg(short, long, default_value = OUTPUT_PATH)]
    pub(crate) output_path: PathBuf,
    /// Czyści plik wyjściowy przed zapisem (domyślnie dopisuje rekordy)
    #[arg(long, default_value_t = false)]
    pub(crate) clear_output: bool,
}

#[derive(Debug, Args)]
pub struct MergeArgs {
    /// Pierwszy plik wejściowy CSV
    pub(crate) first_input: PathBuf,
    /// Drugi plik wejściowy CSV
    pub(crate) second_input: PathBuf,
    /// Ścieżka pliku wyjściowego CSV
    #[arg(short, long, default_value = OUTPUT_PATH)]
    pub(crate) output_path: PathBuf,
    /// Czyści plik wyjściowy przed zapisem (domyślnie dopisuje rekordy)
    #[arg(long, default_value_t = false)]
    pub(crate) clear_output: bool,
}
