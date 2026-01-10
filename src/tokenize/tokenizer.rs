//! WordPiece tokenizer for encoding and decoding text.

use crate::{TokenizerConfig, Vocab};

/// The main tokenizer struct for encoding and decoding text.
///
/// Holds a vocabulary and configuration, providing methods to convert
/// text to token IDs and back.
#[derive(Debug, Clone)]
pub struct Tokenizer {
    /// The vocabulary for token-ID mappings
    vocab: Vocab,
    /// Configuration for encoding behavior
    config: TokenizerConfig,
    /// Prefix for continuing subwords (e.g., "##")
    continuing_subword_prefix: String,
}

impl Tokenizer {
    /// Creates a new tokenizer with the given vocabulary and default configuration.
    pub fn new(vocab: Vocab) -> Self {
        let config = TokenizerConfig::default();
        let continuing_subword_prefix = "##".to_string();
        Tokenizer {
            vocab,
            config,
            continuing_subword_prefix,
        }
    }

    /// Creates a new tokenizer with custom configuration.
    pub fn with_config(vocab: Vocab, config: TokenizerConfig) -> Self {
        let continuing_subword_prefix = "##".to_string();
        Self {
            vocab,
            config,
            continuing_subword_prefix,
        }
    }

    pub fn vocab(&self) -> &Vocab {
        &self.vocab
    }

    pub fn config(&self) -> &TokenizerConfig {
        &self.config
    }
}
