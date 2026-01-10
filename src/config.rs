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

impl SpecialTokens {
    /// Returns all special tokens as a vector.
    pub fn all_tokens(&self) -> Vec<&str> {
        vec![&self.pad, &self.unk, &self.cls, &self.sep, &self.mask]
    }
}

/// Configuration for the tokenizer's encoding behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenizerConfig {
    /// Whether to add [CLS] at start and [SEP] at end (default: true)
    pub wrap_with_cls_sep: bool,
    /// Maximum sequence length to truncate; None means no limit (default: None)
    pub max_length: Option<usize>,
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        TokenizerConfig {
            wrap_with_cls_sep: true,
            max_length: None,
        }
    }
}
