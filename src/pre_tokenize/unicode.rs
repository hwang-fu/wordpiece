//! Unicode-aware pre-tokenization utilities.
//!
//! Handles CJK characters, emojis, and other special Unicode ranges.

/// Checks if a character is a CJK (Chinese, Japanese, Korean) character.
///
/// CJK characters are tokenized individually (each character = one token).
pub fn is_cjk_character(c: char) -> bool {
    // CJK Unified Ideographs and related blocks
    matches!(c,
        '\u{4E00}'..='\u{9FFF}' |   // CJK Unified Ideographs
        '\u{3400}'..='\u{4DBF}' |   // CJK Unified Ideographs Extension A
        '\u{20000}'..='\u{2A6DF}' | // CJK Unified Ideographs Extension B
        '\u{2A700}'..='\u{2B73F}' | // CJK Unified Ideographs Extension C
        '\u{2B740}'..='\u{2B81F}' | // CJK Unified Ideographs Extension D
        '\u{2B820}'..='\u{2CEAF}' | // CJK Unified Ideographs Extension E
        '\u{F900}'..='\u{FAFF}' |   // CJK Compatibility Ideographs
        '\u{2F800}'..='\u{2FA1F}'   // CJK Compatibility Ideographs Supplement
    )
}

/// Checks if a character is a Japanese Hiragana character.
pub fn is_hiragana(c: char) -> bool {
    matches!(c, '\u{3040}'..='\u{309F}')
}

/// Checks if a character is a Japanese Katakana character.
pub fn is_katakana(c: char) -> bool {
    matches!(c, '\u{30A0}'..='\u{30FF}' | '\u{31F0}'..='\u{31FF}')
}
