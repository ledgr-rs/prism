/// Defines the rules and constraints used to score models.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Policy {
    /// The name of the policy.
    pub name: String,
    /// Weights or constraints used during scoring.
    pub constraints: Vec<String>,
}
