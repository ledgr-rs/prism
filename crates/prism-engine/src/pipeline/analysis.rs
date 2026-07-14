use prism_core::error::PrismError;
use prism_core::types::Prompt;

/// Intermediate metadata extracted from a prompt during analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct PromptProfile {
    /// The original raw text.
    pub text: String,
    /// The approximate word count of the prompt.
    pub word_count: usize,
}

/// Responsible for analyzing a normalized prompt to produce a PromptProfile.
pub trait PromptAnalyzer {
    /// Analyzes the prompt and returns its profile.
    fn analyze(&self, prompt: &Prompt) -> Result<PromptProfile, PrismError>;
}

/// A default analyzer that computes basic heuristics (word count).
pub struct DefaultPromptAnalyzer;

impl PromptAnalyzer for DefaultPromptAnalyzer {
    fn analyze(&self, prompt: &Prompt) -> Result<PromptProfile, PrismError> {
        let word_count = prompt.text.split_whitespace().count();
        Ok(PromptProfile {
            text: prompt.text.clone(),
            word_count,
        })
    }
}
