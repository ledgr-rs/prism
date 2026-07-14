use prism_core::error::PrismError;
use prism_core::types::{CapabilityProfile, ModelProfile};

/// Responsible for filtering available models into viable candidates.
///
/// This stage narrows the set of supplied models to those that could
/// feasibly satisfy the extracted capabilities. It performs no scoring.
pub trait CandidateFiltering {
    /// Filters the given candidates and returns the viable subset.
    fn filter(
        &self,
        candidates: Vec<ModelProfile>,
        capabilities: &CapabilityProfile,
    ) -> Result<Vec<ModelProfile>, PrismError>;
}

/// A default filter that passes all models through unchanged.
///
/// Returns an error if the candidate list is empty.
pub struct DefaultCandidateFiltering;

impl CandidateFiltering for DefaultCandidateFiltering {
    fn filter(
        &self,
        candidates: Vec<ModelProfile>,
        _capabilities: &CapabilityProfile,
    ) -> Result<Vec<ModelProfile>, PrismError> {
        if candidates.is_empty() {
            return Err(PrismError::MissingInformation(
                "No candidate models provided".to_string(),
            ));
        }
        Ok(candidates)
    }
}
