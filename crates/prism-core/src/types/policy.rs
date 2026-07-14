/// A single composable policy rule that constrains candidate selection.
///
/// Each rule checks a specific aspect of a model's metadata.
/// Rules are combined via conjunction (all rules must be satisfied).
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyRule {
    /// Maximum cost per token allowed.
    MaxBudget(f64),
    /// Maximum latency in milliseconds allowed.
    MaxLatency(u64),
    /// Only allow models running locally.
    PrivacyLocalOnly,
    /// Only allow models from these providers.
    PreferredProviders(Vec<String>),
    /// Exclude models from these providers.
    ForbiddenProviders(Vec<String>),
    /// Require models to be deployed in a specific region.
    RequiredRegion(String),
    /// Minimum confidence/reliability score (0.0 to 1.0).
    MinConfidence(f64),
}

/// Defines the rules and constraints used to filter candidate models.
///
/// Policies are evaluated by the PolicyEvaluator stage. All rules
/// must be satisfied for a model to be considered eligible.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Policy {
    /// The name of the policy.
    pub name: String,
    /// The composable rules that define this policy.
    pub rules: Vec<PolicyRule>,
}
