use prism_core::error::PrismError;
use prism_core::types::{CandidateScore, ModelProfile, Recommendation};

/// Responsible for selecting the best recommendation from scored candidates.
pub trait DecisionSelector {
    /// Selects the highest-scored candidate as the final recommendation.
    fn select(&self, scored: Vec<(ModelProfile, CandidateScore)>) -> Result<Recommendation, PrismError>;
}

/// A default selector that picks the candidate with the highest score.
pub struct DefaultDecisionSelector;

impl DecisionSelector for DefaultDecisionSelector {
    fn select(&self, scored: Vec<(ModelProfile, CandidateScore)>) -> Result<Recommendation, PrismError> {
        let best = scored
            .into_iter()
            .max_by(|a, b| a.1.final_score.partial_cmp(&b.1.final_score).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or_else(|| PrismError::MissingInformation("No scored candidates available".to_string()))?;

        Ok(Recommendation {
            model: best.0,
            score: best.1.final_score,
        })
    }
}
