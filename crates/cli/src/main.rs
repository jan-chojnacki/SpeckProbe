use clap::Parser;
use cli::application::{generate_config, search};
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
    }

    Ok(())
}
