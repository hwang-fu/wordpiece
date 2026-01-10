//! Plain text vocabulary format.
//!
//! Format: One token per line, line number (0-indexed) equals token ID.
//! Lines starting with '#' are comments (for metadata).

use std::{
    fmt::format,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use crate::{Result, SpecialTokens, Vocab, WordPieceError};

/// Saves a vocabulary to a plain text file.
///
/// Format:
/// ```text
/// # WordPiece vocabulary
/// # vocab_size: 1000
/// [PAD]
/// [UNK]
/// ...
/// ```
///
/// Each token is on its own line, with ID = line number (after comments).
pub fn save_vocab_text<P>(vocab: &Vocab, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write header comments
    writeln!(writer, "# WordPiece vocabulary")?;
    writeln!(writer, "# vocab_size: {}", vocab.len())?;
    writeln!(writer, "#")?;

    // Write tokens (one per line)
    for id in 0..vocab.len() {
        if let Some(token) = vocab.get_token(id) {
            writeln!(writer, "{}", token)?;
        }
    }

    writer.flush()?;

    Ok(())
}

/// Loads a vocabulary from a plain text file.
///
/// Expects one token per line. Lines starting with '#' are ignored.
pub fn load_vocab_text<P>(path: P, special_tokens: SpecialTokens) -> Result<Vocab>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut tokens = Vec::new();
    for line in reader.lines() {
        let line = line?;

        // Skip comment lines
        if line.starts_with('#') {
            continue;
        }

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        tokens.push(line);
    }

    if tokens.is_empty() {
        return Err(WordPieceError::InvalidVocabFile(format!(
            "{} is empty",
            path.display()
        )));
    }

    Ok(Vocab::new(tokens, special_tokens))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::NamedTempFile;

    fn create_test_vocab() -> Vocab {
        let tokens = vec![
            "[PAD]".to_string(),
            "[UNK]".to_string(),
            "[CLS]".to_string(),
            "[SEP]".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        Vocab::new(tokens, SpecialTokens::default())
    }

    #[test]
    fn test_save_vocab_text() {
        let vocab = create_test_vocab();
        let temp_file = NamedTempFile::new().unwrap();

        save_vocab_text(&vocab, temp_file.path()).unwrap();

        // Read and verify
        let mut content = String::new();
        File::open(temp_file.path())
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();

        assert!(content.contains("[PAD]"));
        assert!(content.contains("hello"));
        assert!(content.contains("world"));
    }

    #[test]
    fn test_load_vocab_text() {
        let vocab = create_test_vocab();
        let temp_file = NamedTempFile::new().unwrap();

        // Save then load
        save_vocab_text(&vocab, temp_file.path()).unwrap();
        let loaded = load_vocab_text(temp_file.path(), SpecialTokens::default()).unwrap();

        assert_eq!(loaded.len(), vocab.len());
        assert_eq!(loaded.get_id("[PAD]"), Some(0));
        assert_eq!(loaded.get_id("hello"), Some(4));
    }

    #[test]
    fn test_roundtrip_text() {
        let vocab = create_test_vocab();
        let temp_file = NamedTempFile::new().unwrap();

        save_vocab_text(&vocab, temp_file.path()).unwrap();
        let loaded = load_vocab_text(temp_file.path(), SpecialTokens::default()).unwrap();

        // Verify all tokens match
        for id in 0..vocab.len() {
            assert_eq!(vocab.get_token(id), loaded.get_token(id));
        }
    }
}
