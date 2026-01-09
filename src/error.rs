//! Error types for the WordPiece tokenizer.

use std::{error, fmt, io, result};

/// The main error type for WordPiece operations.
#[derive(Debug)]
pub enum WordPieceError {
    /// File I/O error
    Io(io::Error),
    /// Invalid or malformed vocabulary file
    InvalidVocabFile(String),
    /// Invalid configuration parameter
    InvalidConfig(String),
    /// Error during tokenization/encoding
    Encoding(String),
    /// Error during vocabulary training
    Training(String),
}

impl fmt::Display for WordPieceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WordPieceError::Io(err) => write!(f, "I/O error: {}", err),
            WordPieceError::InvalidVocabFile(msg) => write!(f, "Invalid vocabulary file: {}", msg),
            WordPieceError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            WordPieceError::Encoding(msg) => write!(f, "Encoding error: {}", msg),
            WordPieceError::Training(msg) => write!(f, "Training error: {}", msg),
        }
    }
}

impl From<io::Error> for WordPieceError {
    fn from(err: io::Error) -> Self {
        WordPieceError::Io(err)
    }
}

impl error::Error for WordPieceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            WordPieceError::Io(err) => Some(err),
            _ => None,
        }
    }
}

pub type Result<T> = result::Result<T, WordPieceError>;
