pub mod codec;
pub mod error;
pub mod mode;
pub mod speck;

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
pub enum CipherMode {
    ECB,
    CBC,
}
