//! Training module.

mod alphabet;
mod config;
mod corpus;

pub use alphabet::extract_alphabet
pub use config::TrainingConfig;
pub use corpus::{process_corpus, process_corpus_text};
