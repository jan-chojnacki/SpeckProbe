pub(crate) mod detect_backend;
pub mod helpers;
pub(crate) mod progress_scale;
pub(crate) mod progress_ui;

pub mod args;
pub mod domain;
pub mod enums;

pub use helpers::display_banner;
pub use helpers::display_info;
pub use progress_ui::ProgressUi;

pub const BANNER: &str = r#"
  ____                  _    ____            _
 / ___| _ __   ___  ___| | _|  _ \ _ __ ___ | |__   ___
 \___ \| '_ \ / _ \/ __| |/ / |_) | '__/ _ \| '_ \ / _ \
  ___) | |_) |  __/ (__|   <|  __/| | | (_) | |_) |  __/
 |____/| .__/ \___|\___|_|\_\_|   |_|  \___/|_.__/ \___|
       |_|
"#;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
