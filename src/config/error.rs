use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Invalid chat source: {0}")]
    ChatSource(String),
    #[error("Missing input file: {0}")]
    MissingInputFile(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
pub type Result<T> = std::result::Result<T, ConfigError>;
