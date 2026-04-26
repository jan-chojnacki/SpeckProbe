use clap::Parser;
use cli::application::{benchmark, generate_benchmark_config, generate_config, search};
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
        Commands::Benchmark { config_path } => benchmark::execute(config_path)?,
        Commands::SampleBenchmarkConfig { config_path, force } => {
            generate_benchmark_config::execute(config_path, force)?
        }
    }

    Ok(())
}
