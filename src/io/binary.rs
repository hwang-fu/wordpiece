//! Binary vocabulary format.
//!
//! Format (little-endian):
//! - Magic bytes: "WPVC" (4 bytes)
//! - Version: u8 (1 byte)
//! - Vocab size: u64 (8 bytes)
//! - Tokens: [length: u64, bytes: [u8; length]]...
//! - Checksum: u32 (4 bytes, CRC32 of all preceding bytes)

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{Result, Vocab};

/// "WordPiece Vocabulary" - Magic bytes identifying a WordPiece binary vocabulary file.
const MAGIC: &[u8; 4] = b"WPVC";

/// Current format version.
const VERSION: u8 = 1;

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
