use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MdError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse markdown: {0}")]
    ParseError(String),

    #[error("Theme '{0}' not found. Available themes: {1:?}")]
    ThemeNotFound(String, Vec<String>),

    #[error("Failed to load theme: {0}")]
    ThemeLoadError(String),

    #[error("File watcher error: {0}")]
    WatcherError(#[from] notify::Error),

    #[error("Terminal error: {0}")]
    TerminalError(String),
}
