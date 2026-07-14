use prism_core::error::PrismError;
use prism_core::types::{CapabilityProfile, Explanation, Recommendation};

/// Responsible for generating a human-readable explanation for a recommendation.
pub trait ExplanationGenerator {
    /// Produces an explanation that justifies the given recommendation.
    fn generate(
        &self,
        recommendation: &Recommendation,
        capabilities: &CapabilityProfile,
    ) -> Result<Explanation, PrismError>;
}

/// A default generator that produces a deterministic template-based explanation.
pub struct DefaultExplanationGenerator;

impl ExplanationGenerator for DefaultExplanationGenerator {
    fn generate(
        &self,
        recommendation: &Recommendation,
        capabilities: &CapabilityProfile,
    ) -> Result<Explanation, PrismError> {
        let reasons: Vec<String> = capabilities
            .requirements
            .iter()
            .filter(|req| recommendation.model.capabilities.contains(req))
            .map(|req| format!("supports '{}'", req))
            .collect();

        let reasoning = if reasons.is_empty() {
            format!(
                "Selected '{}' as the best available option.",
                recommendation.model.id
            )
        } else {
            format!(
                "Selected '{}' because it {} (score: {}).",
                recommendation.model.id,
                reasons.join(", "),
                recommendation.score
            )
        };

        Ok(Explanation { reasoning })
    }
}
