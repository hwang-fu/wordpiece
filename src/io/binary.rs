//! Binary vocabulary format.
//!
//! Format (little-endian):
//! - Magic bytes: "WPVC" (4 bytes)
//! - Version: u8 (1 byte)
//! - Vocab size: u64 (8 bytes)
//! - Tokens: [ {length: u64, bytes: [u8; length]} ]
//! - Checksum: u32 (4 bytes, CRC32 of all preceding bytes)

use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use crate::{Result, SpecialTokens, Vocab, WordPieceError};

/// "WordPiece Vocabulary" - Magic bytes identifying a WordPiece binary vocabulary file.
const MAGIC: &[u8; 4] = b"WPVC";

/// Current format version.
const VERSION: u8 = 1;

/// Saves a vocabulary to a binary file.
pub fn save_vocab_binary<P>(vocab: &Vocab, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let mut data = Vec::new();

    // Magic bytes
    data.extend_from_slice(MAGIC);

    // Version
    data.push(VERSION);

    // Vocab size (u64, little-endian)
    let vocab_size = vocab.len() as u64;
    data.extend_from_slice(&vocab_size.to_le_bytes());

    // Tokens: each is [length: u64][bytes]
    for id in 0..vocab.len() {
        if let Some(token) = vocab.get_token(id) {
            let bytes = token.as_bytes();
            let len = bytes.len() as u64;
            data.extend_from_slice(&len.to_le_bytes());
            data.extend_from_slice(bytes);
        }
    }

    // Calculate CRC32 checksum
    let checksum = crc32(&data);
    data.extend_from_slice(&checksum.to_le_bytes());

    writer.write_all(&data)?;
    writer.flush()?;

    Ok(())
}

/// Loads a vocabulary from a binary file.
pub fn load_vocab_binary<P>(path: P, special_tokens: SpecialTokens) -> Result<Vocab>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    // Need at least: magic(4) + version(1) + vocab_size(8) + checksum(4) = 17 bytes
    if data.len() < 17 {
        return Err(WordPieceError::InvalidVocabFile(format!(
            "File {} is too small to be a valid vocabulary",
            path.display()
        )));
    }

    // Verify checksum first (last 4 bytes)
    let checksum_offset = data.len() - 4;
    let stored_checksum = u32::from_le_bytes([
        data[checksum_offset],
        data[checksum_offset + 1],
        data[checksum_offset + 2],
        data[checksum_offset + 3],
    ]);
    let computed_checksum = crc32(&data[..checksum_offset]);
    if stored_checksum != computed_checksum {
        return Err(WordPieceError::InvalidVocabFile(format!(
            "Checksum mismatch - file {} may be corrupted",
            path.display()
        )));
    }

    // Verify magic bytes
    if &data[0..=3] != MAGIC {
        return Err(WordPieceError::InvalidVocabFile(format!(
            "Invalid magic bytes - {} is not a WordPiece vocabulary file",
            path.display()
        )));
    }

    // Check version
    let version = data[4];
    if version != VERSION {
        return Err(WordPieceError::InvalidVocabFile(format!(
            "Unsupported version: {} (expected {})",
            version, VERSION
        )));
    }

    // Read vocab size
    let vocab_size = u64::from_le_bytes([
        data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12],
    ]) as usize;

    // Read tokens
    let mut tokens = Vec::with_capacity(vocab_size);
    let mut offset = 13;

    for _ in 0..vocab_size {
        if offset >= checksum_offset {
            return Err(WordPieceError::InvalidVocabFile(format!(
                "Unexpected end of file in {} while reading tokens",
                path.display()
            )));
        }

        // Parsing the length
        let len = u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]) as usize;
        offset += 8;

        if offset + len > checksum_offset {
            return Err(WordPieceError::InvalidVocabFile(format!(
                "Unexpected end of file in {} while reading tokens",
                path.display()
            )));
        }

        let token = String::from_utf8(data[offset..offset + len].to_vec()).map_err(|_| {
            WordPieceError::InvalidVocabFile(format!("Invalid UTF-8 in file {}", path.display()))
        })?;
        tokens.push(token);
        offset += len;
    }

    Ok(Vocab::new(tokens, special_tokens))
}

/// Simple CRC32 implementation (IEEE polynomial).
fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;

    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }

    !crc
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_vocab() -> Vocab {
        let tokens = vec![
            "[PAD]".to_string(),
            "[UNK]".to_string(),
            "[CLS]".to_string(),
            "[SEP]".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "##ing".to_string(),
        ];
        Vocab::new(tokens, SpecialTokens::default())
    }

    #[test]
    fn test_save_load_binary() {
        let vocab = create_test_vocab();
        let temp_file = NamedTempFile::new().unwrap();

        save_vocab_binary(&vocab, temp_file.path()).unwrap();
        let loaded = load_vocab_binary(temp_file.path(), SpecialTokens::default()).unwrap();

        assert_eq!(loaded.len(), vocab.len());
        for id in 0..vocab.len() {
            assert_eq!(vocab.get_token(id), loaded.get_token(id));
        }
    }

    #[test]
    fn test_binary_magic_validation() {
        let temp_file = NamedTempFile::new().unwrap();

        // Write invalid data
        std::fs::write(temp_file.path(), b"INVALID_DATA_HERE").unwrap();

        let result = load_vocab_binary(temp_file.path(), SpecialTokens::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_binary_checksum_validation() {
        let vocab = create_test_vocab();
        let temp_file = NamedTempFile::new().unwrap();

        save_vocab_binary(&vocab, temp_file.path()).unwrap();

        // Corrupt the file
        let mut data = std::fs::read(temp_file.path()).unwrap();
        if data.len() > 10 {
            data[10] ^= 0xFF; // Flip some bits
        }
        std::fs::write(temp_file.path(), &data).unwrap();

        let result = load_vocab_binary(temp_file.path(), SpecialTokens::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_crc32() {
        // Test vector: "123456789" should give 0xCBF43926
        let data = b"123456789";
        assert_eq!(crc32(data), 0xCBF43926);
    }
}
