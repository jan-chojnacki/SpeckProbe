use clap::Parser;
use cli::application::{benchmark, encrypt, extract_criterion, sample, search};
use cli::presentation::args::{Args, Commands, Sample};

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
        } => extract_criterion::execute(criterion_path, output_path, clear_output)?,
        Commands::Encrypt {
            speck_version,
            cipher_mode,
            key,
            iv,
            data,
        } => {
            encrypt::execute(speck_version, cipher_mode, key.0, iv.map(|iv| iv.0), data)?;
        }
        Commands::Sample { command: sample } => match sample {
            Sample::Search { config_path, force } => sample::search(config_path, force)?,
            Sample::Benchmark { config_path, force } => sample::benchmark(config_path, force)?,
            Sample::Encrypt { config_path, force } => sample::encrypt(config_path, force)?,
        },
    }

    Ok(())
}
