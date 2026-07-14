use prism_core::error::PrismError;
use prism_core::types::{CapabilityProfile, ModelProfile};

/// Responsible for discovering candidate models for a given capability profile.
pub trait CandidateDiscovery {
    /// Returns a list of candidate models that may satisfy the requirements.
    fn discover(&self, capabilities: &CapabilityProfile) -> Result<Vec<ModelProfile>, PrismError>;
}

/// A default discoverer that returns a hardcoded set of candidate models.
pub struct DefaultCandidateDiscovery;

impl CandidateDiscovery for DefaultCandidateDiscovery {
    fn discover(&self, _capabilities: &CapabilityProfile) -> Result<Vec<ModelProfile>, PrismError> {
        Ok(vec![
            ModelProfile {
                id: "gpt-4".to_string(),
                capabilities: vec![
                    "coding".to_string(),
                    "reasoning".to_string(),
                    "general".to_string(),
                ],
            },
            ModelProfile {
                id: "claude-3".to_string(),
                capabilities: vec![
                    "creative writing".to_string(),
                    "reasoning".to_string(),
                    "general".to_string(),
                ],
            },
            ModelProfile {
                id: "gemini-pro".to_string(),
                capabilities: vec![
                    "coding".to_string(),
                    "translation".to_string(),
                    "general".to_string(),
                ],
            },
            ModelProfile {
                id: "llama-3".to_string(),
                capabilities: vec![
                    "general".to_string(),
                    "creative writing".to_string(),
                ],
            },
        ])
    }
}
