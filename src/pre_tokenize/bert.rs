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
