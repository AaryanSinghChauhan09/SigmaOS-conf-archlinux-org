//! Error types for the conf-archlinux-org component.

/// Errors that can occur parsing conf site files.
#[derive(Debug, thiserror::Error)]
pub enum ConfError {
    /// File could not be read.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Front matter or config failed to parse.
    #[error("parse error: {0}")]
    Parse(String),
}
