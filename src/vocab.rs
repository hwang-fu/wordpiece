//! Vocabulary data structure for token-ID mappings.

use std::{collections::HashMap, str};

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

    /// Returns the number of tokens in the vocabulary.
    pub fn len(&self) -> usize {
        self.id_to_token.len()
    }

    /// Returns true if the vocabulary is empty.
    pub fn is_empty(&self) -> bool {
        self.id_to_token.is_empty()
    }

    /// Looks up a token's ID. Returns None if not found.
    pub fn get_id(&self, token: &str) -> Option<usize> {
        self.token_to_id.get(token).copied()
    }

    /// Looks up a token by ID. Returns None if ID is out of range.
    pub fn get_token(&self, id: usize) -> Option<&str> {
        self.id_to_token.get(id).map(|s| s.as_str())
    }

    /// Returns the ID of the unknown token [UNK].
    pub fn unk_id(&self) -> Option<usize> {
        self.get_id(&self.special_tokens.unk)
    }

    /// Returns the ID of the padding token [PAD].
    pub fn pad_id(&self) -> Option<usize> {
        self.get_id(&self.special_tokens.pad)
    }
}
