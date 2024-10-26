use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] crate::config::ConfigError),
    #[error("Parser error: {0}")]
    Parser(#[from] crate::parser::ParseError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
