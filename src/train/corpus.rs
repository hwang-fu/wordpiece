//! Corpus processing for training.

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    Result, normalize_nfc,
    pre_tokenize::{pre_tokenize_bert, split_cjk},
};

/// Processes a corpus file and returns word frequencies.
///
/// Reads the file line by line (memory efficient for large files),
/// applies normalization and pre-tokenization, then counts word frequencies.
///
/// # Arguments
/// * `path` - Path to the corpus file (plain text, one sentence per line)
/// * `lowercase` - Whether to lowercase the text
///
/// # Returns
/// A HashMap mapping words to their frequencies
pub fn process_corpus<P>(path: P, lowercase: bool) -> Result<HashMap<String, usize>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut word_counts = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let normalized = normalize_nfc(&line);
        let tokens = pre_tokenize_bert(&normalized, lowercase);
        let tokens = split_cjk(tokens);

        // Count word frequencies
        for token in tokens.into_iter() {
            *(word_counts.entry(token).or_insert(0)) += 1;
        }
    }

    Ok(word_counts)
}

/// Processes corpus text directly (for testing or in-memory corpora).
///
/// # Arguments
/// * `text` - The corpus text
/// * `lowercase` - Whether to lowercase the text
///
/// # Returns
/// A HashMap mapping words to their frequencies
pub fn process_corpus_text(text: &str, lowercase: bool) -> HashMap<String, usize> {
    let mut word_counts = HashMap::new();

    for line in text.lines() {
        let normalized = normalize_nfc(line);
        let tokens = pre_tokenize_bert(&normalized, lowercase);
        let tokens = split_cjk(tokens);
        for token in tokens {
            *word_counts.entry(token).or_insert(0) += 1;
        }
    }

    word_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_corpus_text_simple() {
        let text = "hello world\nhello rust";
        let counts = process_corpus_text(text, true);

        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("rust"), Some(&1));
    }

    #[test]
    fn test_process_corpus_text_with_punctuation() {
        let text = "Hello, world!";
        let counts = process_corpus_text(text, true);

        assert_eq!(counts.get("hello"), Some(&1));
        assert_eq!(counts.get(","), Some(&1));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("!"), Some(&1));
    }

    #[test]
    fn test_process_corpus_text_cjk() {
        let text = "你好世界";
        let counts = process_corpus_text(text, false);

        assert_eq!(counts.get("你"), Some(&1));
        assert_eq!(counts.get("好"), Some(&1));
        assert_eq!(counts.get("世"), Some(&1));
        assert_eq!(counts.get("界"), Some(&1));
    }

    #[test]
    fn test_process_corpus_text_no_lowercase() {
        let text = "Hello World";
        let counts = process_corpus_text(text, false);

        assert_eq!(counts.get("Hello"), Some(&1));
        assert_eq!(counts.get("World"), Some(&1));
        assert_eq!(counts.get("hello"), None);
    }
}
