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
    char_freq.sort_by(|a, b| {
        let freq_a = &a.1;
        let freq_b = &b.1;
        freq_b.cmp(freq_a)
    }); // Sort by descending order

    // Collect characters from special tokens (these must always be included)
    let special_chars: HashSet<char> = special_tokens
        .all_tokens()
        .iter()
        .flat_map(|token| token.chars())
        .collect();

    let mut alphabet: Vec<char> = Vec::new();
    let mut seen: HashSet<char> = HashSet::new();

    for c in special_chars.into_iter() {
        if alphabet.len() >= limit_alphabet {
            break;
        }
        if seen.insert(c) {
            alphabet.push(c);
        }
    }

    for (c, _freq) in char_freq {
        if alphabet.len() >= limit_alphabet {
            break;
        }
        if seen.insert(c) {
            alphabet.push(c);
        }
    }

    alphabet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_alphabet_simple() {
        let mut word_counts = HashMap::new();
        word_counts.insert("hello".to_string(), 10);
        word_counts.insert("world".to_string(), 5);

        let alphabet = extract_alphabet(&word_counts, 100, &SpecialTokens::default());

        // Should contain all unique characters
        assert!(alphabet.contains(&'h'));
        assert!(alphabet.contains(&'e'));
        assert!(alphabet.contains(&'l'));
        assert!(alphabet.contains(&'o'));
        assert!(alphabet.contains(&'w'));
        assert!(alphabet.contains(&'r'));
        assert!(alphabet.contains(&'d'));
    }

    #[test]
    fn test_extract_alphabet_respects_limit() {
        let mut word_counts = HashMap::new();
        word_counts.insert("abcdefghij".to_string(), 1);

        let alphabet = extract_alphabet(&word_counts, 5, &SpecialTokens::default());

        // Special token chars + limited alphabet chars
        // '[', ']', 'P', 'A', 'D', 'U', 'N', 'K', etc. from special tokens
        // The limit applies to total alphabet size
        assert!(
            alphabet.len() <= 5
                && alphabet.into_iter().all(|c| {
                    // Either within limit or is a special token char
                    "[PAD][UNK][CLS][SEP][MASK]".contains(c)
                })
        );
    }

    #[test]
    fn test_extract_alphabet_includes_special_token_chars() {
        let word_counts = HashMap::new(); // Empty corpus

        let alphabet = extract_alphabet(&word_counts, 100, &SpecialTokens::default());

        // Should include characters from special tokens
        assert!(alphabet.contains(&'['));
        assert!(alphabet.contains(&']'));
    }

    #[test]
    fn test_extract_alphabet_frequency_order() {
        let mut word_counts = HashMap::new();
        word_counts.insert("aaa".to_string(), 10); // 'a' appears 30 times
        word_counts.insert("bb".to_string(), 5); // 'b' appears 10 times

        let special = SpecialTokens {
            pad: String::new(),
            unk: String::new(),
            cls: String::new(),
            sep: String::new(),
            mask: String::new(),
        };

        let alphabet = extract_alphabet(&word_counts, 100, &special);

        // 'a' should come before 'b' due to higher frequency
        let pos_a = alphabet.iter().position(|&c| c == 'a');
        let pos_b = alphabet.iter().position(|&c| c == 'b');

        assert!(pos_a.unwrap() < pos_b.unwrap());
    }
}
