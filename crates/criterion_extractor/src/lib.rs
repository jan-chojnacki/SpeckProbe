use std::error::Error;

pub mod cli;
mod csv_io;
mod discovery;
mod record;

pub use cli::CliArgs;

pub type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn run(args: CliArgs) -> AppResult<()> {
    let result_files = discovery::collect_result_files(&args.criterion_path);
    let all_records = csv_io::read_all_records(&result_files)?;

    csv_io::save_to_csv(&all_records, &args.output_path)?;
    Ok(())
}
