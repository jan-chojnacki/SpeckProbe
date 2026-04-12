use crate::detect_backend::detect_auto_backend;
use colored::Colorize;
use console::{Alignment, pad_str};
use indicatif::{ProgressBar, ProgressStyle};
use primitive_types::U256;
use runtime::api::{BackendHint, CipherConfig, RuntimeConfig, SearchSpace};
use terminal_size::{Width, terminal_size};
use textwrap::{Options, fill};

pub fn display_banner() {
    print!("{}", crate::BANNER);
    let text = format!("{}v{}{}", "(".blue(), crate::VERSION.cyan(), ")".blue());
    println!("{}", pad_str(&text, 69, Alignment::Right, None));
}

fn display_line(text: &str, value: &str, width: usize) -> String {
    let text = format!("{} {}: {}", " •".cyan(), text, value.bright_blue());

    fill(
        &text,
        Options::new(width)
            .initial_indent("")
            .subsequent_indent("    "),
    )
}

fn display_line_additional(text: &str, value: &str, add: &str, width: usize) -> String {
    let text = format!(
        "{} {}: {} {}",
        " •".cyan(),
        text,
        value.bright_blue(),
        add.blue()
    );

    fill(
        &text,
        Options::new(width)
            .initial_indent("")
            .subsequent_indent("    "),
    )
}

pub fn display_info(
    cipher_config: CipherConfig,
    runtime_config: RuntimeConfig,
    search_space: SearchSpace,
) {
    let cipher_mode = cipher_config.cipher_mode.to_string();
    let speck_version = cipher_config.speck_version.to_string();
    let num_threads = runtime_config.num_threads.to_string();
    let num_threads_add = format!("({}/{})", num_threads, num_cpus::get());
    let backend_hint = runtime_config.backend_hint.to_string();
    let backend_detected = format!("({})", detect_auto_backend().to_string());

    let mut start = search_space.start;
    let mut end = search_space.end;

    for _ in 0..runtime_config.suffix_bytes_size {
        start.insert(0, 0);
        end.insert(0, u8::MAX);
    }

    let start: String = start.iter().map(|b| format!("{:02x} ", b)).collect();
    let end: String = end.iter().map(|b| format!("{:02x} ", b)).collect();

    let width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    println!("{}", display_line("Cipher mode", &cipher_mode, width));
    println!("{}", display_line("Speck version", &speck_version, width));
    println!(
        "{}",
        display_line_additional("Thread count", &num_threads, &num_threads_add, width)
    );
    match runtime_config.backend_hint {
        BackendHint::Auto => println!(
            "{}",
            display_line_additional("Selected backend", &backend_hint, &backend_detected, width)
        ),
        _ => println!("{}", display_line("Selected backend", &backend_hint, width)),
    }
    println!("{}", display_line("Start", &start, width));
    println!("{}", display_line("End", &end, width));
    println!();
}

pub fn build_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    let style = ProgressStyle::with_template(
        "{spinner:.yellow} [{elapsed_precise}] [{bar:36.cyan/blue}] {percent}% | {msg} ({eta})",
    )
    .unwrap()
    .progress_chars("=>-");
    pb.set_style(style);
    pb
}

pub fn le_to_u256(bytes: &[u8]) -> U256 {
    let mut buf = [0u8; 32];
    buf[..bytes.len()].copy_from_slice(bytes);
    U256::from_little_endian(&buf)
}

pub fn significant_bytes(value: U256) -> u32 {
    let le = value.to_little_endian();
    le.iter()
        .rposition(|&b| b != 0)
        .map(|i| i as u32 + 1)
        .unwrap_or(1)
}
