use crate::types::{Prompt, CapabilityProfile, Recommendation, Explanation};

/// A complete report of the decision process.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DecisionReport {
    /// The original prompt.
    pub prompt: Prompt,
    /// The analyzed capabilities.
    pub capabilities: CapabilityProfile,
    /// The final recommendation.
    pub recommendation: Recommendation,
    /// The explanation for the recommendation.
    pub explanation: Explanation,
}
