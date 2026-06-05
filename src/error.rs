use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to load file: {0}")]
    FileLoad(#[from] std::io::Error),

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Network error occurred")]
    Network,
}
