/// Represents the requirements extracted from a prompt.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CapabilityProfile {
    /// A list of required capabilities (e.g., "coding", "creative writing").
    pub requirements: Vec<String>,
}
