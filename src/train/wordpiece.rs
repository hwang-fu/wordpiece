//! WordPiece training algorithm.

use std::collections::HashMap;

use crate::{Vocab, train::TrainingConfig};

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

impl WordPieceTrainer {
    /// Creates a new trainer with the given configuration.
    pub fn new(config: TrainingConfig) -> Self {
        Self { config }
    }

    /// Trains a vocabulary from word frequencies.
    ///
    /// # Arguments
    /// * `word_counts` - Map of words to their frequencies
    /// * `alphabet` - Initial character alphabet
    ///
    /// # Returns
    /// A trained `Vocab` ready for tokenization
    pub fn train(&self, word_counts: &HashMap<String, usize>, alphabet: Vec<char>) -> Vocab {
        // Initialize vocabulary with special tokens and alphabet
        let mut vocab_tokens: Vec<String> = Vec::new();

        // Add special tokens first
        let special_tokens = self.config.get_special_tokens();
        for special_token in special_tokens.all_tokens() {
            if !special_token.is_empty() && !vocab_tokens.contains(&special_token.to_string()) {
                vocab_tokens.push(special_token.to_string());
            }
        }

        // Add alphabet characters (as single-char tokens)
        for c in &alphabet {
            let token = c.to_string();
            if !vocab_tokens.contains(&token) {
                vocab_tokens.push(token);
            }
        }

        // Convert words to symbol sequences, filtering by `min_frequency` configured
        let min_frequency = self.config.get_min_frequency();

        panic!("not finished yet");
    }

    /// Converts a word into initial symbol sequence.
    ///
    /// First character stays as-is, subsequent characters get the
    /// continuing subword prefix (e.g., "##").
    fn word_to_symbols(&self, word: &str) -> Vec<String> {
        let prefix = self.config.get_continuing_subword_prefix();
        let mut symbols = Vec::new();

        for (i, c) in word.chars().enumerate() {
            if i == 0 {
                symbols.push(c.to_string());
            } else {
                symbols.push(format!("{}{}", prefix, c));
            }
        }

        symbols
    }

    /// Counts all adjacent symbol pairs across all words.
    fn count_pairs(&self, words: &[Word]) -> HashMap<(String, String), usize> {
        let mut pair_counts = HashMap::new();

        for word in words {
            if word.symbols.len() < 2 {
                continue;
            }

            let word_len = word.symbols.len();
            for i in 0..word_len - 1 {
                let pair = (word.symbols[i].clone(), word.symbols[i + 1].clone());
                *(pair_counts.entry(pair).or_insert(0)) += word.count;
            }
        }

        pair_counts
    }
}
