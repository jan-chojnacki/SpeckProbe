use crate::domain::backend::detect_auto_backend;
use colored::Colorize;
use console::{Alignment, pad_str};
use runtime::api::{BackendHint, CipherConfig, DispatchOutput, RuntimeConfig, SearchSpace};
use std::fmt::Display;
use terminal_size::{Width, terminal_size};
use textwrap::{Options, fill};

struct Line {
    prefix: String,
    label: Option<String>,
    value: Option<String>,
    suffix: Option<String>,
}

impl Line {
    fn new(prefix: impl Display) -> Self {
        Self {
            prefix: prefix.to_string(),
            label: None,
            value: None,
            suffix: None,
        }
    }

    fn label(mut self, l: impl Display) -> Self {
        self.label = Some(l.to_string());
        self
    }

    fn value(mut self, v: impl Display) -> Self {
        self.value = Some(v.to_string());
        self
    }

    fn suffix(mut self, s: impl Display) -> Self {
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

pub fn display_banner() {
    print!("{}", crate::BANNER);
    let text = format!("{}v{}{}", "(".blue(), crate::VERSION.cyan(), ")".blue());
    println!("{}", pad_str(&text, 57, Alignment::Right, None));
}

pub fn display_results(results: DispatchOutput, spurious: bool) {
    let width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    if spurious {
        println!(
            "{}",
            Line::new(" •".cyan())
                .label("Spurious key/s:")
                .render(width)
        );
        for (idx, key) in results.0.iter().enumerate() {
            let key: String = key.iter().map(|b| format!("{:02x} ", b)).collect();
            println!(
                "{}",
                Line::new("   ")
                    .label(idx.to_string().cyan())
                    .value(key)
                    .render(width)
            );
        }
    }

    match results.1 {
        None => {
            let count = results.0.len().to_string();
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
        Some(key) => {
            let key: String = key.iter().map(|b| format!("{:02x} ", b)).collect();
            println!(
                "{}",
                Line::new(" •".green())
                    .label("Found key")
                    .value(key)
                    .render(width)
            );
        }
    }
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
    let backend_detected = format!("({})", detect_auto_backend());

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
        bullet()
            .label("Speck version")
            .value(&speck_version)
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
}
