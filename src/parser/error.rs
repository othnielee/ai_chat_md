use std::num::ParseFloatError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse timestamp: {0}")]
    TimestampError(#[from] chrono::ParseError),
    #[error("Failed to parse float: {0}")]
    FloatError(#[from] ParseFloatError),
    #[error("Invalid timestamp value: {0}")]
    InvalidTimestamp(f64),
    #[error("Format error: {0}")]
    Format(#[from] std::fmt::Error),
}

pub type Result<T> = std::result::Result<T, ParseError>;
