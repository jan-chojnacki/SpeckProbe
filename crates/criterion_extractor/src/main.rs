use clap::Parser;

fn main() -> criterion_extractor::AppResult<()> {
    let args = criterion_extractor::CliArgs::parse();
    criterion_extractor::run(args)
}
