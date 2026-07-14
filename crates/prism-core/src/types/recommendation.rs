use crate::types::ModelProfile;

/// Represents a recommended model for a given prompt.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Recommendation {
    /// The profile of the recommended model.
    pub model: ModelProfile,
    /// The score assigned to this recommendation.
    pub score: f64,
}
