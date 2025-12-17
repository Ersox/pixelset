use thiserror::Error;

#[derive(Debug, Error)]
pub enum ColorParseError {
    #[error("hex code must be 6 or 8 characters long, got {0}")]
    InvalidLength(usize),
    #[error("invalid hexadecimal value for {0}: {1}")]
    InvalidHex(&'static str, #[source] std::num::ParseIntError),
}