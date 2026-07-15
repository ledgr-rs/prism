use crate::types::{
    CandidateScore, CapabilityProfile, Explanation, ModelProfile, Policy, Prompt, PromptProfile,
    Recommendation,
};

/// A complete report of the decision process.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DecisionReport {
    /// The original prompt.
    pub prompt: Prompt,
    /// Prompt analysis produced by the engine.
    pub prompt_profile: PromptProfile,
    /// The analyzed capabilities.
    pub capabilities: CapabilityProfile,
    /// Model profiles supplied to the engine for this decision.
    pub model_registry: Vec<ModelProfile>,
    /// Candidates after capability filtering.
    pub candidates: Vec<ModelProfile>,
    /// Candidates that passed policy evaluation.
    pub policy_approved_candidates: Vec<ModelProfile>,
    /// The policy used during the decision.
    pub policy: Policy,
    /// Scored candidates produced by the scoring stage.
    pub scored_candidates: Vec<(ModelProfile, CandidateScore)>,
    /// The final recommendation.
    pub recommendation: Recommendation,
    /// The explanation for the recommendation.
    pub explanation: Explanation,
}
