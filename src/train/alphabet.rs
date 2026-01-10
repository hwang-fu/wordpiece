//! Initial alphabet extraction for WordPiece training.

use std::collections::{HashMap, HashSet};

use crate::SpecialTokens;

/// Extracts the initial alphabet from word frequencies.
///
/// The alphabet consists of individual characters that appear in the corpus,
/// sorted by frequency and limited to `limit_alphabet` characters.
///
/// # Arguments
/// * `word_counts` - Word frequency map from corpus processing
/// * `limit_alphabet` - Maximum number of characters to include
/// * `special_tokens` - Special tokens whose characters must be included
///
/// # Returns
/// A sorted vector of unique characters (most frequent first)
pub fn extract_alphabet(
    word_counts: &HashMap<String, usize>,
    limit_alphabet: usize,
    special_tokens: &SpecialTokens,
) -> Vec<char> {
    // Count character frequencies across all words
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for (word, &count) in word_counts {
        for c in word.chars() {
            *(char_counts.entry(c).or_insert(0)) += count;
        }
    }

    let mut char_freq: Vec<(char, usize)> = char_counts.into_iter().collect();
    char_freq.sort_by(|a, b| a.1.cmp(&(b.1)));

    // Collect characters from special tokens (these must always be included)
    let special_chars: HashSet<char> = special_tokens
        .all_tokens()
        .iter()
        .flat_map(|token| token.chars())
        .collect();

    let mut alphabet: Vec<char> = Vec::new();
    let mut seen: HashSet<char> = HashSet::new();
    for c in special_chars.into_iter() {
        if seen.insert(c) {
            alphabet.push(c);
        }
    }

    for (c, _freq) in char_freq {
        if seen.insert(c) {
            alphabet.push(c);
            if alphabet.len() >= limit_alphabet {
                break;
            }
        }
    }

    alphabet
}
