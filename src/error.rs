//! Error types for the WordPiece tokenizer.

use std::io;

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
