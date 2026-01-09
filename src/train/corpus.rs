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
