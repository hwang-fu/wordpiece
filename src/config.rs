//! Configuration structures for the WordPiece tokenizer.

/// Represents the set of special tokens used by the tokenizer.
///
/// These tokens have reserved meanings and are typically placed
/// at the beginning of the vocabulary.
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
