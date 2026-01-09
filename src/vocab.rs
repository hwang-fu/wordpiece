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

impl Vocab {
    /// Creates a new vocabulary from a list of tokens.
    ///
    /// Tokens are assigned IDs in order (0, 1, 2, ...).
    /// Special tokens should be included at the beginning of the list.
    pub fn new(tokens: Vec<String>, special_tokens: SpecialTokens) -> Self {
        let mut token_to_id = HashMap::with_capacity(tokens.len());
        for (id, token) in tokens.iter().enumerate() {
            token_to_id.insert(token.clone(), id);
        }

        let id_to_token = tokens;

        Vocab {
            token_to_id,
            id_to_token,
            special_tokens,
        }
    }
}
