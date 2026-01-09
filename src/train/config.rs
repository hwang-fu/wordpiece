//! Training configuration for the WordPiece algorithm.

use crate::SpecialTokens;

/// Configuration for WordPiece vocabulary training.
///
/// All parameters have sensible defaults compatible with BERT.
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    /// Target vocabulary size (default: 30,000)
    vocab_size: usize,
    /// Minimum frequency for a token to be included (default: 2)
    min_frequency: usize,
    /// Maximum size of the initial character alphabet (default: 1,000)
    limit_alphabet: usize,
    /// Prefix for continuing subwords, e.g., "##" in "play" -> "play", "##ing" (default: "##")
    continuing_subword_prefix: String,
    /// Whether to lowercase text during training (default: true)
    lowercase: bool,
    /// Special tokens to include in vocabulary (default: BERT-style)
    special_tokens: SpecialTokens,
}

impl TrainingConfig {
    /// Creates a new TrainingConfig with default values.
    pub fn new() -> Self {
        TrainingConfig::default()
    }

    /// Sets the target vocabulary size.
    pub fn set_vocab_size(mut self, size: usize) -> Self {
        if size == 0 {
            panic!("target vocabulary size is supposed to be greater than zero")
        }
        self.vocab_size = size;
        self
    }

    /// Gets the target vocabulary size.
    pub fn get_vocab_size(&self) -> usize {
        self.vocab_size
    }

    /// Sets the minimum token frequency threshold.
    pub fn set_min_frequency(mut self, freq: usize) -> Self {
        if freq == 0 {
            panic!("target token frequency is supposed to be greater than zero")
        }
        self.min_frequency = freq;
        self
    }

    /// Gets the minimum token frequency threshold.
    pub fn get_min_frequency(&self) -> usize {
        self.min_frequency
    }

    /// Sets the maximum initial alphabet size.
    pub fn set_limit_alphabet(mut self, limit: usize) -> Self {
        if limit == 0 {
            panic!("maximum initial alphabet size is supposed to be greater than zero")
        }
        self.limit_alphabet = limit;
        self
    }

    /// Gets the maximum initial alphabet size.
    pub fn get_limit_alphabet(&self) -> usize {
        self.limit_alphabet
    }

    /// Sets the continuing subword prefix.
    pub fn set_continuing_subword_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.continuing_subword_prefix = prefix.into();
        self
    }

    /// Gets the continuing subword prefix.
    pub fn get_continuing_subword_prefix(&self) -> &str {
        self.continuing_subword_prefix.as_str()
    }

    /// Sets whether to lowercase text.
    pub fn lowercase(mut self, lowercase: bool) -> Self {
        self.lowercase = lowercase;
        self
    }

    /// Gets whether to lowercase text.
    pub fn get_lowercase(&self) -> bool {
        self.lowercase
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
