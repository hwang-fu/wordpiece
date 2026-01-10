//! Binary vocabulary format.
//!
//! Format (little-endian):
//! - Magic bytes: "WPVC" (4 bytes)
//! - Version: u8 (1 byte)
//! - Vocab size: u32 (4 bytes)
//! - Tokens: [length: u32, bytes: [u8; length]]...
//! - Checksum: u32 (4 bytes, CRC32 of all preceding bytes)

/// Magic bytes identifying a WordPiece binary vocabulary file.
const MAGIC: &[u8; 4] = b"WPVC";

/// Current format version.
const VERSION: u8 = 1;

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
