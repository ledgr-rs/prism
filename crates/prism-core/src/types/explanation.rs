/// Provides the reasoning behind a specific recommendation.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Explanation {
    /// The textual explanation of why the model was chosen.
    pub reasoning: String,
}
