use prism_core::error::PrismError;
use prism_core::types::Prompt;

/// Responsible for normalizing a raw prompt before analysis.
pub trait Normalizer {
    /// Normalizes the given prompt for further processing.
    fn normalize(&self, prompt: &Prompt) -> Result<Prompt, PrismError>;
}

/// A default normalizer that returns the prompt unchanged.
pub struct DefaultNormalizer;

impl Normalizer for DefaultNormalizer {
    fn normalize(&self, prompt: &Prompt) -> Result<Prompt, PrismError> {
        Ok(prompt.clone())
    }
}
