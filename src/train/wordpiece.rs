//! WordPiece training algorithm.

use crate::train::TrainingConfig;

/// Represents a word split into subword units during training.
#[derive(Debug, Clone)]
struct Word {
    /// The subword units (initially individual characters)
    symbols: Vec<String>,
    /// How many times this word appears in the corpus
    count: usize,
}

/// WordPiece trainer that builds vocabulary from corpus.
pub struct WordPieceTrainer {
    config: TrainingConfig,
}
