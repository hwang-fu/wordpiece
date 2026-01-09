//! Vocabulary data structure for token-ID mappings.

use std::collections::HashMap;

use crate::config::SpecialTokens;

/// A vocabulary mapping between tokens and their IDs.
///
/// Maintains bidirectional mappings for efficient lookup in both directions.
#[derive(Debug, Clone)]
pub struct Vocab {
    /// Maps token strings to their numeric IDs
    token_to_id: HashMap<String, usize>,
    /// Maps numeric IDs back to token strings
    id_to_token: Vec<String>,
    /// Special tokens configuration
    special_tokens: SpecialTokens,
}
