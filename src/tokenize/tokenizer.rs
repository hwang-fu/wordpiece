//! WordPiece tokenizer for encoding and decoding text.

use crate::{
    TokenizerConfig, Vocab, normalize_nfc,
    pre_tokenize::{pre_tokenize_bert, split_cjk},
};

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

    /// Encodes text into token IDs.
    ///
    /// This is the main encoding method that applies the full pipeline:
    /// 1. Unicode normalization (NFC)
    /// 2. Pre-tokenization (whitespace + punctuation splitting)
    /// 3. CJK character splitting
    /// 4. WordPiece tokenization
    /// 5. Add special tokens (if configured)
    /// 6. Truncation (if configured)
    ///
    /// # Arguments
    /// * `text` - The text to encode
    ///
    /// # Returns
    /// A vector of token IDs
    pub fn encode(&self, text: &str) -> Vec<usize> {
        // Step 1: Normalization
        let normalized = normalize_nfc(text);

        // Step 2 & 3: Pre-tokenize
        let lowercase = false; // Could be made configurable in the future
        let pre_tokens = pre_tokenize_bert(&normalized, lowercase);
        let pre_tokens = split_cjk(pre_tokens);

        // Step 4: WordPiece tokenization
        let mut token_ids = Vec::new();
        for pre_token in pre_tokens {
            let token = self.tokenize_word(&pre_token);
            token_ids.extend(token);
        }

        // Step 5: Add special tokens
        if self.config.add_special_tokens {
            token_ids = self.add_special_tokens(token_ids);
        }

        // Step 6: Truncation
        if self.config.truncation
            && let Some(max_len) = self.config.max_length
        {
            token_ids.truncate(max_len);
        }

        token_ids
    }

    /// Encodes text into token strings (useful for debugging).
    ///
    /// Same as `encode` but returns token strings instead of IDs.
    pub fn encode_to_tokens(&self, text: &str) -> Vec<String> {
        let ids = self.encode(text);
        ids.iter()
            .filter_map(|&id| self.vocab.get_token(id).map(|s| s.to_string()))
            .collect()
    }

    /// Tokenizes a single word using the WordPiece algorithm.
    ///
    /// Attempts to find the longest matching prefix in the vocabulary,
    /// then continues with the remainder using the subword prefix.
    fn tokenize_word(&self, word: &str) -> Vec<usize> {
        let mut tokens = Vec::new();
        if word.is_empty() {
            return tokens;
        }

        // Get the [UNK] token id for unknown subwords.
        // Panic if vocab doesn't have [UNK] — this is a configuration error.
        let unk_id = self.vocab.unk_id().expect("vocab must have [UNK] token");

        // Convert to Vec<char> for proper Unicode handling.
        // This allows us to slice by character index, not byte index.
        let chars: Vec<char> = word.chars().collect();
        let mut start = 0;

        // Reusable buffer to avoid allocations in the inner loop.
        // Pre-allocate enough space for the word + prefix (e.g., "##").
        let mut buffer = String::with_capacity(word.len() + self.continuing_subword_prefix.len());

        // Greedy longest-match loop:
        // For each position, try to find the longest substring that exists in vocab.
        while start < chars.len() {
            // Search from longest to shortest substring (greedy matching).
            // `(start + 1..=chars.len()).rev()` generates: [len, len-1, ..., start+1]
            let matched = (start + 1..=chars.len()).rev().find_map(|end| {
                buffer.clear();

                // For continuation tokens (not the first piece), prepend the
                // subword prefix (typically "##" for BERT-style tokenizers).
                if start > 0 {
                    buffer.push_str(&self.continuing_subword_prefix);
                }
                buffer.extend(&chars[start..end]);

                // Return (token_id, end_position) if found in vocab.
                self.vocab.get_id(&buffer).map(|id| (id, end))
            });

            match matched {
                Some((id, end)) => {
                    tokens.push(id);
                    start = end;
                }
                None => {
                    tokens.push(unk_id);
                    start += 1;
                }
            }
        }

        tokens
    }

    /// Adds [CLS] and [SEP] tokens around the token IDs.
    fn add_special_tokens(&self, mut token_ids: Vec<usize>) -> Vec<usize> {
        let cls_id = self.vocab.cls_id();
        let sep_id = self.vocab.sep_id();

        if let Some(cls) = cls_id {
            token_ids.insert(0, cls);
        }
        if let Some(sep) = sep_id {
            token_ids.push(sep);
        }

        token_ids
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

    /// Returns the continuing subword prefix.
    pub fn get_continuing_subword_prefix(&self) -> &str {
        &self.continuing_subword_prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SpecialTokens;

    fn create_test_tokenizer() -> Tokenizer {
        let tokens = vec![
            "[PAD]".to_string(),
            "[UNK]".to_string(),
            "[CLS]".to_string(),
            "[SEP]".to_string(),
            "[MASK]".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "##ing".to_string(),
        ];
        let vocab = Vocab::new(tokens, SpecialTokens::default());
        Tokenizer::new(vocab)
    }

    #[test]
    fn test_tokenizer_new() {
        let tokenizer = create_test_tokenizer();
        assert_eq!(tokenizer.vocab().len(), 8);
        assert_eq!(tokenizer.continuing_subword_prefix(), "##");
    }

    #[test]
    fn test_tokenizer_with_config() {
        let tokens = vec!["[UNK]".to_string()];
        let vocab = Vocab::new(tokens, SpecialTokens::default());
        let config = TokenizerConfig {
            add_special_tokens: false,
            max_length: Some(512),
            truncation: true,
        };
        let tokenizer = Tokenizer::with_config(vocab, config);

        assert!(!tokenizer.config().add_special_tokens);
        assert_eq!(tokenizer.config().max_length.as_ref(), Some(&512));
    }
}
