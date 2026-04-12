pub(crate) mod detect_backend;
pub(crate) mod helpers;
pub(crate) mod progress_scale;
pub(crate) mod progress_ui;

pub use helpers::display_banner;
pub use helpers::display_info;
pub use progress_ui::ProgressUi;

pub const BANNER: &str = r#"
 .▄▄ ·  ▄▄▄·▄▄▄ . ▄▄· ▄ •▄      ▄▄· ▄▄▄   ▄▄▄·  ▄▄· ▄ •▄ ▄▄▄ .▄▄▄
 ▐█ ▀. ▐█ ▄█▀▄.▀·▐█ ▌▪█▌▄▌▪    ▐█ ▌▪▀▄ █·▐█ ▀█ ▐█ ▌▪█▌▄▌▪▀▄.▀·▀▄ █·
 ▄▀▀▀█▄ ██▀·▐▀▀▪▄██ ▄▄▐▀▀▄·    ██ ▄▄▐▀▀▄ ▄█▀▀█ ██ ▄▄▐▀▀▄·▐▀▀▪▄▐▀▀▄
 ▐█▄▪▐█▐█▪·•▐█▄▄▌▐███▌▐█.█▌    ▐███▌▐█•█▌▐█ ▪▐▌▐███▌▐█.█▌▐█▄▄▌▐█•█▌
  ▀▀▀▀ .▀    ▀▀▀ ·▀▀▀ ·▀  ▀    ·▀▀▀ .▀  ▀ ▀  ▀ ·▀▀▀ ·▀  ▀ ▀▀▀ .▀  ▀
"#;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
