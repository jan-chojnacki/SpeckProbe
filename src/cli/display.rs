use crate::benchmark::BenchmarkConfig;
use crate::search::executor::CipherMode::Cbc;
use crate::search::executor::{
    BackendHint, CipherConfig, DispatchOutput, RuntimeConfig, SearchSpace,
};
use colored::Colorize;
use console::{Alignment, pad_str};
use std::fmt;
use std::path::Path;
use terminal_size::{Width, terminal_size};
use textwrap::{Options, fill};

struct Line {
    prefix: String,
    label: Option<String>,
    value: Option<String>,
    suffix: Option<String>,
}

impl Line {
    fn new(prefix: impl fmt::Display) -> Self {
        Self {
            prefix: prefix.to_string(),
            label: None,
            value: None,
            suffix: None,
        }
    }

    fn label(mut self, l: impl fmt::Display) -> Self {
        self.label = Some(l.to_string());
        self
    }

    fn value(mut self, v: impl fmt::Display) -> Self {
        self.value = Some(v.to_string());
        self
    }

    fn suffix(mut self, s: impl fmt::Display) -> Self {
        self.suffix = Some(s.to_string());
        self
    }

    fn render(&self, width: usize) -> String {
        let text = match (&self.label, &self.value, &self.suffix) {
            (Some(l), None, None) => format!("{} {}", self.prefix, l),
            (Some(l), Some(v), None) => {
                format!("{} {}: {}", self.prefix, l, v.as_str().bright_blue())
            }
            (Some(l), Some(v), Some(s)) => format!(
                "{} {}: {} {}",
                self.prefix,
                l,
                v.as_str().bright_blue(),
                s.as_str().blue()
            ),
            (_, _, _) => self.prefix.clone(),
        };
        fill(
            &text,
            Options::new(width)
                .initial_indent("")
                .subsequent_indent("    "),
        )
    }
}

/// Formats a byte slice as space-separated lowercase hex pairs (e.g. `"0a 1b 2c"`).
fn format_key_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Joins an iterator of `Display` items with `", "`.
fn join_display<T: fmt::Display>(items: impl Iterator<Item = T>) -> String {
    items.map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
}

/// Prepends `count` copies of `fill` in front of `key`.
fn prepend_bytes(key: Vec<u8>, count: usize, fill: u8) -> Vec<u8> {
    let mut result = vec![fill; count];
    result.extend(key);
    result
}

pub fn display_banner() {
    print!("{}", super::BANNER);
    let text = format!("{}v{}{}", "(".blue(), super::VERSION.cyan(), ")".blue());
    println!("{}", pad_str(&text, 57, Alignment::Right, None));
}

fn print_spurious_keys(keys: &[Vec<u8>], width: usize) {
    println!(
        "{}",
        Line::new(" •".cyan())
            .label("Spurious key/s:")
            .render(width)
    );
    for (idx, key) in keys.iter().enumerate() {
        println!(
            "{}",
            Line::new("   ")
                .label(idx.to_string().cyan())
                .value(format_key_hex(key))
                .render(width)
        );
    }
}

fn print_key_not_found(count: usize, width: usize) {
    println!(
        "{}",
        Line::new(" •".red())
            .label(format!(
                "Key not found, found {} spurious key/s",
                count.to_string().blue()
            ))
            .render(width)
    );
}

fn print_key_found(key: &[u8], width: usize) {
    println!(
        "{}",
        Line::new(" •".green())
            .label("Found key")
            .value(format_key_hex(key))
            .render(width)
    );
}

pub fn display_results(results: DispatchOutput, spurious: bool) {
    let width = terminal_width();
    if spurious {
        print_spurious_keys(&results.0, width);
    }
    match &results.1 {
        None => print_key_not_found(results.0.len(), width),
        Some(key) => print_key_found(key, width),
    }
}

pub fn display_info(
    cipher_config: CipherConfig,
    runtime_config: RuntimeConfig,
    search_space: SearchSpace,
) {
    let cipher_mode = cipher_config.cipher_mode.to_string();
    let operation = cipher_config.cipher_function.to_string();
    let speck_version = cipher_config.speck_version.to_string();
    let suffix_bytes = runtime_config.suffix_bytes_size.to_string();
    let num_threads = runtime_config.num_threads.to_string();
    let num_threads_add = format!("({}/{})", num_threads, num_cpus::get());
    let backend_hint = runtime_config.backend_hint.to_string();
    let backend_detected = format!("({})", detect());

    let start = format_key_hex(&prepend_bytes(
        search_space.start,
        runtime_config.suffix_bytes_size,
        0,
    ));
    let end = format_key_hex(&prepend_bytes(
        search_space.end,
        runtime_config.suffix_bytes_size,
        u8::MAX,
    ));

    let width = terminal_width();
    let bullet = || Line::new(" •".cyan());

    println!(
        "{}",
        bullet()
            .label("Cipher mode")
            .value(&cipher_mode)
            .render(width)
    );
    println!(
        "{}",
        bullet().label("Operation").value(&operation).render(width)
    );
    println!(
        "{}",
        bullet()
            .label("Speck version")
            .value(&speck_version)
            .render(width)
    );
    println!(
        "{}",
        bullet()
            .label("Suffix bytes")
            .value(&suffix_bytes)
            .render(width)
    );
    println!(
        "{}",
        bullet()
            .label("Thread count")
            .value(&num_threads)
            .suffix(&num_threads_add)
            .render(width)
    );
    match runtime_config.backend_hint {
        BackendHint::Auto => println!(
            "{}",
            bullet()
                .label("Selected backend")
                .value(&backend_hint)
                .suffix(&backend_detected)
                .render(width)
        ),
        _ => println!(
            "{}",
            bullet()
                .label("Selected backend")
                .value(&backend_hint)
                .render(width)
        ),
    }
    println!("{}", bullet().label("Start").value(&start).render(width));
    println!("{}", bullet().label("End").value(&end).render(width));

    if cipher_config.cipher_mode == Cbc {
        let iv_bytes: Vec<u8> = search_space
            .iv
            .unwrap()
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect();
        let iv = format_key_hex(&iv_bytes);
        println!("{}", bullet().label("IV").value(&iv).render(width));
    }
}

pub fn display_benchmark_info(config: &BenchmarkConfig, output_path: &Path, total_passes: usize) {
    let width = terminal_width();
    let bullet = || Line::new(" •".cyan());

    let versions = join_display(config.speck_versions.iter());
    let modes = join_display(config.cipher_modes.iter());
    let functions = join_display(config.cipher_functions.iter());
    let backends = join_display(config.backend_hints.iter());
    let suffixes = join_display(config.suffix_bytes_values.iter());

    println!(
        "{}",
        bullet()
            .label("Bits")
            .value(config.bits.to_string())
            .render(width)
    );
    println!(
        "{}",
        bullet()
            .label("Speck versions")
            .value(&versions)
            .render(width)
    );
    println!(
        "{}",
        bullet().label("Cipher modes").value(&modes).render(width)
    );
    println!(
        "{}",
        bullet()
            .label("Cipher functions")
            .value(&functions)
            .render(width)
    );
    println!(
        "{}",
        bullet().label("Backends").value(&backends).render(width)
    );
    println!(
        "{}",
        bullet()
            .label("Suffix bytes")
            .value(&suffixes)
            .render(width)
    );
    println!(
        "{}",
        bullet()
            .label("Total passes")
            .value(total_passes.to_string())
            .render(width)
    );
    println!(
        "{}",
        bullet()
            .label("Output")
            .value(output_path.display().to_string())
            .render(width)
    );
}

fn terminal_width() -> usize {
    terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80)
}

/// The SIMD backend that will be selected at executor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveBackend {
    #[cfg(target_arch = "x86_64")]
    Avx512,
    #[cfg(target_arch = "x86_64")]
    Avx2,
    #[cfg(target_arch = "x86_64")]
    Sse2,
    #[cfg(target_arch = "aarch64")]
    Neon,
    Scalar,
}

impl fmt::Display for ActiveBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(target_arch = "x86_64")]
            ActiveBackend::Avx512 => write!(f, "AVX-512"),
            #[cfg(target_arch = "x86_64")]
            ActiveBackend::Avx2 => write!(f, "AVX2"),
            #[cfg(target_arch = "x86_64")]
            ActiveBackend::Sse2 => write!(f, "SSE2"),
            #[cfg(target_arch = "aarch64")]
            ActiveBackend::Neon => write!(f, "NEON"),
            ActiveBackend::Scalar => write!(f, "Scalar"),
        }
    }
}

/// Detects the best available SIMD backend for the current CPU at executor.
fn detect() -> ActiveBackend {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512bw") {
            return ActiveBackend::Avx512;
        }
        if is_x86_feature_detected!("avx2") {
            return ActiveBackend::Avx2;
        }
        if is_x86_feature_detected!("sse2") {
            return ActiveBackend::Sse2;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("neon") {
            return ActiveBackend::Neon;
        }
    }
    ActiveBackend::Scalar
}
