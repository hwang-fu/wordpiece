//! Training module.

mod config;
mod corpus;

pub use config::TrainingConfig;
pub use corpus::{process_corpus, process_corpus_text};
