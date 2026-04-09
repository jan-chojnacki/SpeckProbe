use crate::cli::{CliArgs, Command, ExtractArgs, MergeArgs};
use crate::{AppResult, csv_io, discovery, record};

pub fn run(args: CliArgs) -> AppResult<()> {
    match args.command {
        Command::Extract(extract_args) => run_extract(extract_args),
        Command::Merge(merge_args) => run_merge(merge_args),
    }
}

fn run_extract(args: ExtractArgs) -> AppResult<()> {
    let architecture = record::current_architecture();
    let result_files = discovery::collect_result_files(&args.criterion_path);
    let all_records = csv_io::read_all_records(&result_files, &architecture)?;
    csv_io::save_to_csv(&all_records, &args.output_path, args.clear_output)?;
    Ok(())
}

fn run_merge(args: MergeArgs) -> AppResult<()> {
    csv_io::merge_csv_files(&args.first_input, &args.second_input)?;
    Ok(())
}
