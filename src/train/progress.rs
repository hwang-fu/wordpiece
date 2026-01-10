//! Progress reporting for training.

/// Information about the current training progress.
#[derive(Debug, Clone)]
pub struct TrainingPogress {
    /// Current vocabulary size
    pub current_vocab_size: usize,
    /// Target vocabulary size
    pub target_vocab_size: usize,
    /// Current iteration number
    pub iteration: usize,
    /// The pair that was just merged (if any)
    pub merged_pair: Option<(String, String)>,
    /// The resulting merged token (if any)
    pub merged_token: Option<String>,
}
