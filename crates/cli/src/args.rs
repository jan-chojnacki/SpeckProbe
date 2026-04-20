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
        config_path: PathBuf,
        #[arg(short, long)]
        spurious: bool,
    },
    SampleConfig {
        config_path: PathBuf,
        #[arg(short, long)]
        force: bool,
    },
}
