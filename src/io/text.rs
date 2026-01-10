//! Plain text vocabulary format.
//!
//! Format: One token per line, line number (0-indexed) equals token ID.
//! Lines starting with '#' are comments (for metadata).

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{Result, Vocab};

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
