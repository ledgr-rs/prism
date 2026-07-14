use prism_core::error::PrismError;
use prism_core::types::CapabilityProfile;

use crate::pipeline::analysis::PromptProfile;

/// Responsible for extracting a CapabilityProfile from a PromptProfile.
pub trait CapabilityExtractor {
    /// Extracts required capabilities from the analyzed prompt.
    fn extract(&self, profile: &PromptProfile) -> Result<CapabilityProfile, PrismError>;
}

/// A default extractor that uses simple keyword matching.
pub struct DefaultCapabilityExtractor;

impl CapabilityExtractor for DefaultCapabilityExtractor {
    fn extract(&self, profile: &PromptProfile) -> Result<CapabilityProfile, PrismError> {
        let text = profile.intrinsic.text.to_lowercase();
        let mut requirements = Vec::new();

        if text.contains("code") || text.contains("program") || text.contains("function") {
            requirements.push("coding".to_string());
        }
        if text.contains("write") || text.contains("story") || text.contains("creative") {
            requirements.push("creative writing".to_string());
        }
        if text.contains("analy") || text.contains("reason") || text.contains("logic") {
            requirements.push("reasoning".to_string());
        }
        if text.contains("translate") || text.contains("language") {
            requirements.push("translation".to_string());
        }

        if requirements.is_empty() {
            requirements.push("general".to_string());
        }

        Ok(CapabilityProfile { requirements })
    }
}
