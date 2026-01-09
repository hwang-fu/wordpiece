//! BERT-style pre-tokenization.
//!
//! Splits text on whitespace and punctuation, following the original
//! BERT tokenizer's approach.

/// Checks if a character is a punctuation character.
///
/// Uses a simple heuristic: ASCII punctuation plus Unicode punctuation categories.
fn is_punctuation(c: char) -> bool {
    // ASCII punctuation
    if c.is_ascii_punctuation() {
        return true;
    }

    // Unicode general category: check if it's a punctuation character
    // Categories: Pc, Pd, Pe, Pf, Pi, Po, Ps
    matches!(c,
        '\u{00A1}'..='\u{00BF}' |  // Latin-1 punctuation
        '\u{2000}'..='\u{206F}' |  // General punctuation
        '\u{2E00}'..='\u{2E7F}' |  // Supplemental punctuation
        '\u{3000}'..='\u{303F}'    // CJK symbols and punctuation
    )
}

/// Checks if a character is a whitespace character.
fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

/// Pre-tokenizes text using BERT-style rules.
///
/// This function:
/// 1. Splits on whitespace
/// 2. Splits punctuation into separate tokens
/// 3. Optionally lowercases the text
///
/// # Arguments
/// * `text` - The input text to pre-tokenize
/// * `lowercase` - Whether to lowercase the output tokens
///
/// # Returns
/// A vector of pre-tokenized strings (words and punctuation separated)
pub fn pre_tokenize_bert(text: &str, lowercase: bool) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for c in text.chars() {
        if is_whitespace(c) {
            // Whitespace: flush current token and skip the whitespace
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
        } else if is_punctuation(c) {
            // Punctuation: flush current token, then add punctuation as separate token
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
            tokens.push(c.to_string());
        } else {
            // Regular character: accumulate
            current_token.push(c);
        }
    }

    // Don't forget the last token
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    // Apply lowercasing if requested
    if lowercase {
        tokens
            .into_iter()
            .map(|token: String| token.to_lowercase())
            .collect()
    } else {
        tokens
    }
}
