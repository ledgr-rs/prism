use prism_core::error::PrismError;

use crate::pipeline::intrinsic::IntrinsicProfile;

/// Inferred properties derived from a prompt's intrinsic observations.
///
/// These represent conclusions drawn from the IntrinsicProfile rather than
/// directly observable facts.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedProfile {
    /// The inferred category of the task.
    pub task_category: String,
    /// Estimated complexity level.
    pub complexity: String,
    /// Required depth of reasoning.
    pub reasoning_depth: String,
    /// Estimated ambiguity of the request.
    pub ambiguity: String,
}

/// Responsible for analyzing intrinsic observations to produce a DerivedProfile.
pub trait DerivedAnalyzer {
    /// Analyzes the intrinsic profile and infers derived properties.
    fn analyze(&self, intrinsic: &IntrinsicProfile) -> Result<DerivedProfile, PrismError>;
}

/// A default analyzer that uses simple heuristics on intrinsic fields.
pub struct DefaultDerivedAnalyzer;

impl DerivedAnalyzer for DefaultDerivedAnalyzer {
    fn analyze(&self, intrinsic: &IntrinsicProfile) -> Result<DerivedProfile, PrismError> {
        let lower = intrinsic.text.to_lowercase();

        let task_category = if intrinsic.modality == "code" || !intrinsic.languages.is_empty() {
            "coding".to_string()
        } else if lower.contains("write") || lower.contains("story") || lower.contains("creative") {
            "creative writing".to_string()
        } else if lower.contains("analy") || lower.contains("compare") || lower.contains("evaluate") {
            "analysis".to_string()
        } else if lower.contains("translate") || lower.contains("language") {
            "translation".to_string()
        } else if lower.contains("summar") || lower.contains("explain") {
            "summarization".to_string()
        } else {
            "general".to_string()
        };

        let complexity = if intrinsic.word_count > 200 {
            "high".to_string()
        } else if intrinsic.word_count > 50 {
            "medium".to_string()
        } else {
            "low".to_string()
        };

        let reasoning_depth = if lower.contains("explain")
            || lower.contains("why")
            || lower.contains("compare")
            || lower.contains("analyze")
        {
            "deep".to_string()
        } else if lower.contains("what") || lower.contains("how") || lower.contains("describe") {
            "moderate".to_string()
        } else {
            "shallow".to_string()
        };

        let ambiguity = if lower.contains("maybe")
            || lower.contains("perhaps")
            || lower.contains("either")
            || lower.contains("or")
            || lower.contains("might")
        {
            "high".to_string()
        } else if lower.ends_with('?') && lower.starts_with("what") || lower.starts_with("how") {
            "medium".to_string()
        } else {
            "low".to_string()
        };

        Ok(DerivedProfile {
            task_category,
            complexity,
            reasoning_depth,
            ambiguity,
        })
    }
}
