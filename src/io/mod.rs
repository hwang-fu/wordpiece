//! I/O (persistence) module.

mod binary;
mod text;

pub use binary::{load_vocab_binary, save_vocab_binary};
pub use text::{load_vocab_text, save_vocab_text};
