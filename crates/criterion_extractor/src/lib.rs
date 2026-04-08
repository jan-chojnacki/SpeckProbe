use std::error::Error;

mod app;
pub mod cli;
mod csv_io;
mod discovery;
mod record;

pub use app::run;
pub use cli::CliArgs;

pub type AppResult<T> = Result<T, Box<dyn Error>>;
