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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfc_precomposed() {
        // Already precomposed - should remain unchanged
        let input = "café";
        assert_eq!(normalize_nfc(input), "café");
    }

    #[test]
    fn test_nfc_decomposed() {
        // Decomposed 'é' (e + combining acute accent) -> precomposed 'é'
        let input = "cafe\u{0301}"; // 'e' followed by combining acute accent
        let output = normalize_nfc(input);
        assert_eq!(output, "café");
        assert_eq!(output.len(), 5); // 'c' 'a' 'f' 'é'(2 bytes) = 5 bytes
    }

    #[test]
    fn test_nfc_mixed() {
        // Mix of ASCII and decomposed characters
        let input = "re\u{0301}sume\u{0301}"; // résumé with decomposed accents
        assert_eq!(normalize_nfc(input), "résumé");
    }
}
