use prism_core::error::PrismError;
use prism_core::types::{CapabilityProfile, ModelProfile};

/// Responsible for scoring candidate models against capability requirements.
pub trait CandidateScorer {
    /// Scores each candidate model and returns a list of (model, score) pairs.
    fn score(
        &self,
        candidates: Vec<ModelProfile>,
        capabilities: &CapabilityProfile,
    ) -> Result<Vec<(ModelProfile, f64)>, PrismError>;
}

/// A default scorer that counts capability overlap as the score.
pub struct DefaultCandidateScorer;

impl CandidateScorer for DefaultCandidateScorer {
    fn score(
        &self,
        candidates: Vec<ModelProfile>,
        capabilities: &CapabilityProfile,
    ) -> Result<Vec<(ModelProfile, f64)>, PrismError> {
        let scored: Vec<(ModelProfile, f64)> = candidates
            .into_iter()
            .map(|model| {
                let overlap = capabilities
                    .requirements
                    .iter()
                    .filter(|req| model.capabilities.contains(req))
                    .count();
                (model, overlap as f64)
            })
            .collect();

        Ok(scored)
    }
}
