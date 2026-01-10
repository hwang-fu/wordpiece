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

impl TrainingPogress {
    /// Returns the progress as a percentage (0.0 to 100.0).
    pub fn percentage(&self) -> f64 {
        if self.target_vocab_size == 0 {
            100.0
        } else {
            (self.current_vocab_size as f64 / self.target_vocab_size as f64) * 100.0
        }
    }
}
