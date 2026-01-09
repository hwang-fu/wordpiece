//! Training configuration for the WordPiece algorithm.

use crate::SpecialTokens;

/// Configuration for WordPiece vocabulary training.
///
/// All parameters have sensible defaults compatible with BERT.
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    /// Target vocabulary size (default: 30,000)
    pub vocab_size: usize,
    /// Minimum frequency for a token to be included (default: 2)
    pub min_frequency: usize,
    /// Maximum size of the initial character alphabet (default: 1,000)
    pub limit_alphabet: usize,
    /// Prefix for continuing subwords, e.g., "##" in "play" -> "play", "##ing" (default: "##")
    pub continuing_subword_prefix: String,
    /// Whether to lowercase text during training (default: true)
    pub lowercase: bool,
    /// Special tokens to include in vocabulary (default: BERT-style)
    pub special_tokens: SpecialTokens,
}

impl TrainingConfig {
    /// Creates a new TrainingConfig with default values.
    pub fn new() -> Self {
        TrainingConfig::default()
    }

    /// Sets the target vocabulary size.
    pub fn set_vocab_size(mut self, size: usize) -> Self {
        self.vocab_size = size;
        self
    }
}

impl Default for TrainingConfig {
    fn default() -> Self {
        TrainingConfig {
            vocab_size: 30_000,
            min_frequency: 2,
            limit_alphabet: 1_000,
            continuing_subword_prefix: "##".to_string(),
            lowercase: true,
            special_tokens: SpecialTokens::default(),
        }
    }
}
