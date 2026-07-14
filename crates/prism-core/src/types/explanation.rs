/// A piece of evidence supporting the recommendation.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Evidence {
    /// Description of the evidence.
    pub detail: String,
    /// Where the evidence originated.
    pub source: String,
}

/// Whether a capability requirement was satisfied by the selected model.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchStatus {
    /// The model fully supports this capability.
    Fulfilled,
    /// The model partially supports this capability.
    Partial,
    /// The model does not support this capability.
    Unfulfilled,
}

impl Default for MatchStatus {
    fn default() -> Self {
        MatchStatus::Unfulfilled
    }
}

/// Describes whether a specific capability requirement was met.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CapabilityMatch {
    /// The capability that was checked.
    pub capability: String,
    /// Whether the selected model fulfills this capability.
    pub status: MatchStatus,
    /// Why the capability was or was not matched.
    pub reason: String,
}

/// Records how a policy constraint affected the decision.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PolicyDecision {
    /// The policy or constraint evaluated.
    pub policy: String,
    /// Whether the policy was applied.
    pub applied: bool,
    /// Explanation of the policy decision.
    pub reason: String,
}

/// A rejected candidate model and the reason it was not selected.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RejectedAlternative {
    /// The identifier of the rejected model.
    pub model_id: String,
    /// The score the rejected model received.
    pub score: f64,
    /// Why this candidate was rejected.
    pub reason: String,
}

/// Provides structured reasoning behind a specific recommendation.
///
/// Unlike a flat text blob, this type models the decision as structured data
/// so that consumers (CLI, TUI, API) can render explanations without
/// re-running inference or parsing text templates.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Explanation {
    /// A concise summary of the recommendation.
    pub summary: String,
    /// Supporting evidence that informed the decision.
    pub evidence: Vec<Evidence>,
    /// How each capability requirement was matched by the selected model.
    pub capability_matches: Vec<CapabilityMatch>,
    /// How policy constraints affected the decision.
    pub policy_decisions: Vec<PolicyDecision>,
    /// Rejected candidate models with reasons.
    pub rejected_alternatives: Vec<RejectedAlternative>,
    /// Overall confidence in the recommendation (0.0 – 1.0).
    pub confidence: f64,
}
