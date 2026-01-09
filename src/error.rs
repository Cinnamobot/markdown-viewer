use thiserror::Error;

#[derive(Error, Debug)]
pub enum MdvError {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Invalid markdown syntax at line {line}: {msg}")]
    ParseError { line: usize, msg: String },

    #[error("Syntax highlighting failed: {0}")]
    HighlightError(String),

    #[error("File watcher error: {0}")]
    WatcherError(#[from] notify::Error),

    #[error("Terminal error: {0}")]
    TerminalError(String),
}

pub type Result<T> = std::result::Result<T, MdvError>;
