//! Unicode normalization utilities.

use unicode_normalization::UnicodeNormalization;

/// Normalizes text to Unicode NFC (Canonical Composition) form.
///
/// NFC normalization ensures that equivalent Unicode sequences
/// are represented identically. For example, 'é' can be represented as:
/// - A single code point U+00E9 (precomposed)
/// - Two code points U+0065 U+0301 (decomposed: 'e' + combining accent)
///
/// NFC converts both to the precomposed form.
pub fn normalize_nfc(text: &str) -> String {
    text.nfc().collect()
}
