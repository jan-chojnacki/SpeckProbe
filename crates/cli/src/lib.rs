pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub const BANNER: &str = r#"
  ____                  _    ____            _
 / ___| _ __   ___  ___| | _|  _ \ _ __ ___ | |__   ___
 \___ \| '_ \ / _ \/ __| |/ / |_) | '__/ _ \| '_ \ / _ \
  ___) | |_) |  __/ (__|   <|  __/| | | (_) | |_) |  __/
 |____/| .__/ \___|\___|_|\_\_|   |_|  \___/|_.__/ \___|
       |_|
"#;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
