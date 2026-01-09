//! Configuration structures for the WordPiece tokenizer.

/// Represents the set of special tokens used by the tokenizer.
///
/// These tokens have reserved meanings and are typically placed
/// at the beginning of the vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialTokens {
    /// Padding token (default: "[PAD]")
    pub pad: String,
    /// Unknown token for out-of-vocabulary words (default: "[UNK]")
    pub unk: String,
    /// Classification token, marks sequence start (default: "[CLS]")
    pub cls: String,
    /// Separator token, marks sequence boundary (default: "[SEP]")
    pub sep: String,
    /// Mask token for masked language modeling (default: "[MASK]")
    pub mask: String,
}

impl Default for SpecialTokens {
    fn default() -> Self {
        SpecialTokens {
            pad: "[PAD]".to_string(),
            unk: "[UNK]".to_string(),
            cls: "[CLS]".to_string(),
            sep: "[SEP]".to_string(),
            mask: "[MASK]".to_string(),
        }
    }
}

/// Configuration for the tokenizer's encoding behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenizerConfig {
    /// Whether to add [CLS] at start and [SEP] at end (default: true)
    pub add_special_tokens: bool,
    /// Maximum sequence length; None means no limit
    pub max_length: Option<usize>,
    /// Whether to truncate sequences exceeding max_length (default: true)
    pub truncation: bool,
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        TokenizerConfig {
            add_special_tokens: true,
            max_length: None,
            truncation: true,
        }
    }
}
