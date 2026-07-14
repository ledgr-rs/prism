use crate::types::{CapabilityProfile, Prompt};
use crate::error::PrismError;

/// Responsible for converting a Prompt into a CapabilityProfile.
pub trait Analyzer {
    /// Analyzes the given prompt to determine the required capabilities.
    fn analyze(&self, prompt: &Prompt) -> Result<CapabilityProfile, PrismError>;
}
