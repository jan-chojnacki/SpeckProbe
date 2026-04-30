use clap::Parser;
use speck_probe::cli::{
    Args, Commands, SampleCommand, benchmark, encrypt, extract_criterion, sample, search,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Search {
            config_path,
            spurious,
        } => search::execute(config_path, spurious)?,
        Commands::Benchmark {
            config_path,
            output_path,
        } => benchmark::execute(config_path, output_path)?,
        Commands::ExtractCriterion {
            criterion_path,
            output_path,
            clear_output,
        } => extract_criterion::handle_extract(criterion_path, output_path, clear_output)?,
        Commands::Encrypt {
            speck_version,
            cipher_mode,
            key,
            iv,
            data,
        } => encrypt::handle_encrypt(speck_version, cipher_mode, key.0, iv.map(|iv| iv.0), data)?,
        Commands::Sample { command: s } => match s {
            SampleCommand::Search { config_path, force } => {
                sample::handle_sample_search(config_path, force)?
            }
            SampleCommand::Benchmark { config_path, force } => {
                sample::handle_sample_benchmark(config_path, force)?
            }
        },
    }

    Ok(())
}
