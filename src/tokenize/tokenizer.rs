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
        Tokenizer {
            vocab,
            config,
            continuing_subword_prefix,
        }
    }

    /// Creates a new tokenizer with the given vocabulary and custom configuration and prefix.
    pub fn from(vocab: Vocab, config: TokenizerConfig, prefix: impl Into<String>) -> Self {
        let continuing_subword_prefix = prefix.into();
        Tokenizer {
            vocab,
            config,
            continuing_subword_prefix,
        }
    }

    /// Sets the continuing subword prefix.
    pub fn set_continuing_subword_prefix(&mut self, prefix: impl Into<String>) {
        self.continuing_subword_prefix = prefix.into();
    }

    /// Returns a reference to the vocabulary.
    pub fn vocab(&self) -> &Vocab {
        &self.vocab
    }

    /// Returns a reference to the configuration.
    pub fn config(&self) -> &TokenizerConfig {
        &self.config
    }

    /// Returns the continuing subword prefix.
    pub fn continuing_subword_prefix(&self) -> &str {
        &self.continuing_subword_prefix
    }
}
