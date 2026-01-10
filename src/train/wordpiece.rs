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
        let special_tokens = self.config.get_special_tokens().clone();
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
        let mut words: Vec<Word> = word_counts
            .iter()
            .filter(|&(_word, &freq)| freq >= min_frequency)
            .map(|(word, &count)| {
                let symbols = self.word_to_symbols(word);
                Word { symbols, count }
            })
            .collect();

        // Iteratively merge best pairs until vocab_size is reached
        let vocab_size = self.config.get_vocab_size();
        while vocab_tokens.len() < vocab_size {
            // Count all adjacent pairs
            let pair_counts = self.count_pairs(&words);
            if pair_counts.is_empty() {
                break;
            }

            // Find the best pair (with the highest score)
            let best_pair = self.find_best_pair(&pair_counts);
            if best_pair.is_none() {
                break;
            }

            let (left, right) = best_pair.unwrap();
            let merged = format!("{}{}", left, right);

            // Update all words by merging this pair
            self.merge_pair(&mut words, &left, &right, &merged);

            // Add merged token to vocabulary (if not present)
            if !vocab_tokens.contains(&merged) {
                vocab_tokens.push(merged);
            }
        }

        Vocab::new(vocab_tokens, special_tokens)
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
        let mut pair_freq = HashMap::new();

        for word in words {
            if word.symbols.len() < 2 {
                continue;
            }

            let word_len = word.symbols.len();
            for i in 0..word_len - 1 {
                let pair = (word.symbols[i].clone(), word.symbols[i + 1].clone());
                *(pair_freq.entry(pair).or_insert(0)) += word.count;
            }
        }

        pair_freq
    }

    /// Finds the best pair to merge using WordPiece scoring.
    ///
    /// Score = pair_count / (count(left) * count(right))
    /// This favors pairs where both parts frequently appear together.
    fn find_best_pair(
        &self,
        pair_freq: &HashMap<(String, String), usize>,
    ) -> Option<(String, String)> {
        let mut best_pair = None;
        let mut best_score = f64::NEG_INFINITY;

        // First, count individual symbol frequencies
        let mut symbol_freq = HashMap::new();
        for ((left, right), &freq) in pair_freq.iter() {
            *(symbol_freq.entry(left).or_insert(0)) += freq;
            *(symbol_freq.entry(right).or_insert(0)) += freq;
        }

        // Then, find pair with the highest score
        for ((left, right), freq) in pair_freq.iter() {
            let left_freq = symbol_freq.get(left).expect(
                format!(
                    "{} is supposed to have a frequency greater than or equal to 1",
                    left
                )
                .as_str(),
            );
            let left_freq = *left_freq as f64;

            let right_freq = symbol_freq.get(right).expect(
                format!(
                    "{} is supposed to have a frequency greater than or equal to 1",
                    right
                )
                .as_str(),
            );

            let right_freq = *right_freq as f64;

            let freq = *freq as f64;

            let score = freq / (left_freq * right_freq);
            if score > best_score {
                best_score = score;
                best_pair = Some((left.clone(), right.clone()));
            }
        }

        best_pair
    }

    /// Merges a pair of symbols into all words.
    fn merge_pair(&self, words: &mut [Word], left: &str, right: &str, merged: &str) {
        for word in words.iter_mut() {
            let mut i = 0;
            while i < word.symbols.len().saturating_sub(1) {
                if word.symbols[i] == left && word.symbols[i + 1] == right {
                    word.symbols[i] = merged.to_string();
                    word.symbols.remove(i + 1);
                    // Don't increment i - check if we can merge again at same position
                } else {
                    i += 1;
                }
            }
        }
    }
}
