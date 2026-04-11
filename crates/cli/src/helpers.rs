use indicatif::{ProgressBar, ProgressStyle};
use primitive_types::U256;

pub fn build_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    let style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
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
