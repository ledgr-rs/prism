use prism_core::error::PrismError;
use prism_core::types::{CapabilityProfile, ModelProfile, Policy};

/// Responsible for applying policy constraints to filter candidate models.
pub trait PolicyEvaluator {
    /// Filters the given candidates using the provided policy and capability requirements.
    fn evaluate(
        &self,
        candidates: Vec<ModelProfile>,
        capabilities: &CapabilityProfile,
        policy: &Policy,
    ) -> Result<Vec<ModelProfile>, PrismError>;
}

/// A default evaluator that filters models by capability overlap.
pub struct DefaultPolicyEvaluator;

impl PolicyEvaluator for DefaultPolicyEvaluator {
    fn evaluate(
        &self,
        candidates: Vec<ModelProfile>,
        _capabilities: &CapabilityProfile,
        _policy: &Policy,
    ) -> Result<Vec<ModelProfile>, PrismError> {
        let filtered: Vec<ModelProfile> = candidates
            .into_iter()
            .filter(|m| {
                if _policy.constraints.is_empty() {
                    return true;
                }
                _policy.constraints.iter().all(|c| m.capabilities.contains(c))
            })
            .collect();

        if filtered.is_empty() {
            return Err(PrismError::MissingInformation(
                "No candidates satisfy the active policy".to_string(),
            ));
        }

        Ok(filtered)
    }
}
