//! WordPiece training algorithm.

use core::panic;
use std::collections::HashMap;

use crate::{
    Vocab,
    train::{
        TrainingConfig, {NoOpProgress, ProgressCallback, TrainingProgress},
    },
};

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
        self.train_with_progress(word_counts, alphabet, &mut NoOpProgress)
    }

    pub fn train_with_progress<P>(
        &self,
        word_counts: &HashMap<String, usize>,
        alphabet: Vec<char>,
        progress: &mut P,
    ) -> Vocab
    where
        P: ProgressCallback,
    {
        let mut vocab_tokens = Vec::new();

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

        let initial_vocab_size = vocab_tokens.len();
        let target_vocab_size = self.config.get_vocab_size();
        // Notify progress start
        progress.on_start(target_vocab_size, initial_vocab_size);

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

        let mut iteration = 0;

        while vocab_tokens.len() < target_vocab_size {
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
                vocab_tokens.push(merged.clone());
            }

            // Report progress
            iteration += 1;
            let current_vocab_size = vocab_tokens.len();
            let merged_pair = Some((left, right));
            let merged_token = Some(merged);
            progress.on_progress(&TrainingProgress {
                current_vocab_size,
                target_vocab_size,
                iteration,
                merged_pair,
                merged_token,
            });
        }

        // Notify complete
        progress.on_complete(vocab_tokens.len());

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
            let left_freq = symbol_freq.get(left).unwrap_or_else(|| {
                panic!(
                    "{} is supposed to have a frequency greater than or equal to 1",
                    left
                )
            });
            let left_freq = *left_freq as f64;

            let right_freq = symbol_freq.get(right).unwrap_or_else(|| {
                panic!(
                    "{} is supposed to have a frequency greater than or equal to 1",
                    right
                )
            });

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

/// Convenience function to train a vocabulary from a corpus file.
///
/// This combines all training steps:
/// 1. Process corpus to get word frequencies
/// 2. Extract initial alphabet
/// 3. Train WordPiece vocabulary
///
/// # Arguments
/// * `corpus_path` - Path to the corpus file
/// * `config` - Training configuration
///
/// # Returns
/// A trained `Vocab` or an error
pub fn train_from_file<P: AsRef<std::path::Path>>(
    corpus_path: P,
    config: TrainingConfig,
) -> crate::Result<Vocab> {
    use super::{extract_alphabet, process_corpus};

    // Step 1: Process corpus
    let word_counts = process_corpus(corpus_path, config.get_lowercase())?;

    // Step 2: Extract alphabet
    let alphabet = extract_alphabet(
        &word_counts,
        config.get_limit_alphabet(),
        config.get_special_tokens(),
    );

    // Step 3: Train vocabulary
    let trainer = WordPieceTrainer::new(config);
    let vocab = trainer.train(&word_counts, alphabet);

    Ok(vocab)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_to_symbols() {
        let config = TrainingConfig::new();
        let trainer = WordPieceTrainer::new(config);

        let symbols = trainer.word_to_symbols("hello");
        assert_eq!(symbols, vec!["h", "##e", "##l", "##l", "##o"]);
    }

    #[test]
    fn test_train_simple() {
        let config = TrainingConfig::new()
            .set_vocab_size(20)
            .set_min_frequency(1);
        let trainer = WordPieceTrainer::new(config);

        let mut word_counts = HashMap::new();
        word_counts.insert("low".to_string(), 5);
        word_counts.insert("lower".to_string(), 2);
        word_counts.insert("newest".to_string(), 6);
        word_counts.insert("widest".to_string(), 3);

        // let alphabet: Vec<char> = "a".chars().collect();
        let alphabet: Vec<char> = "lownerstwidest".chars().collect();
        let vocab = trainer.train(&word_counts, alphabet);

        // Should have created a vocabulary
        assert!(!vocab.is_empty());
        // Should contain some characters from the alphabet
        assert!(vocab.get_id("l").is_some() || vocab.get_id("##l").is_some());
    }

    #[test]
    fn test_count_pairs() {
        let config = TrainingConfig::new();
        let trainer = WordPieceTrainer::new(config);

        let words = vec![
            Word {
                symbols: vec!["a".to_string(), "##b".to_string()],
                count: 3,
            },
            Word {
                symbols: vec!["a".to_string(), "##b".to_string()],
                count: 2,
            },
        ];

        let pairs = trainer.count_pairs(&words);
        assert_eq!(pairs.get(&("a".to_string(), "##b".to_string())), Some(&5));
    }
}
