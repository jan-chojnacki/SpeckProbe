use crate::search::executor::CipherMode;
use crate::speck::SpeckVersion;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
#[command(infer_subcommands = true)]
pub enum Commands {
    Search {
        #[arg(default_value = "./config/search.toml")]
        config_path: PathBuf,
        #[arg(short, long)]
        spurious: bool,
    },
    Encrypt {
        #[arg(short, long)]
        speck_version: SpeckVersion,
        #[arg(short, long)]
        cipher_mode: CipherMode,
        #[arg(short, long)]
        key: HexBytes,
        #[arg(short, long)]
        iv: Option<HexBytes>,
        data: String,
    },
    Sample {
        #[clap(subcommand)]
        command: SampleCommand,
    },
    ExtractCriterion {
        #[arg(short = 'i', long, default_value = "./target/criterion/")]
        criterion_path: PathBuf,
        #[arg(short = 'o', long, default_value = "./output/criterion.csv")]
        output_path: PathBuf,
        #[arg(short, long)]
        clear_output: bool,
    },
}

#[derive(Debug, Subcommand, Clone)]
#[command(infer_subcommands = true)]
pub enum SampleCommand {
    Search {
        #[arg(default_value = "./config/search.toml")]
        config_path: PathBuf,
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Clone, Debug)]
pub struct HexBytes(pub Vec<u8>);

impl std::str::FromStr for HexBytes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).map_err(|e| e.to_string()))
            .collect::<Result<Vec<u8>, _>>()
            .map(HexBytes)
    }
}
