use crate::types::{CapabilityProfile, ModelProfile, Policy};
use crate::error::PrismError;

/// Responsible for scoring a model given a CapabilityProfile, ModelProfile, and Policy.
pub trait Scorer {
    /// Calculates a score for the model based on the required capabilities and the active policy.
    fn score(&self, capability: &CapabilityProfile, model: &ModelProfile, policy: &Policy) -> Result<f64, PrismError>;
}
