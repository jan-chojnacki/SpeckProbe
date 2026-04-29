mod args;
mod command;
mod display;
mod progress;

const BANNER: &str = r#"
  ____                  _    ____            _
 / ___| _ __   ___  ___| | _|  _ \ _ __ ___ | |__   ___
 \___ \| '_ \ / _ \/ __| |/ / |_) | '__/ _ \| '_ \ / _ \
  ___) | |_) |  __/ (__|   <|  __/| | | (_) | |_) |  __/
 |____/| .__/ \___|\___|_|\_\_|   |_|  \___/|_.__/ \___|
       |_|
"#;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use command::benchmark;
pub use command::encrypt;
pub use command::extract_criterion;
pub use command::sample;
pub use command::search;

pub use args::Args;
pub use args::Commands;
pub use args::SampleCommand;
