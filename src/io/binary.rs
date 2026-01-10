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
