use prism_core::error::PrismError;

use crate::pipeline::analysis::PromptProfile;

/// Execution requirements inferred from a PromptProfile.
///
/// These are implementation-neutral descriptions of what the task
/// requires, before mapping to Prism's canonical capability taxonomy.
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct TaskRequirements {
    /// Whether the task requires code generation or modification.
    pub needs_code_generation: bool,
    /// Whether the task requires structured or formatted output.
    pub needs_structured_output: bool,
    /// Whether the task requires multi-step reasoning.
    pub needs_reasoning: bool,
    /// Whether the task requires translation between languages.
    pub needs_translation: bool,
    /// Whether the task requires long-form writing.
    pub needs_writing: bool,
    /// Whether the task requires planning across multiple steps.
    pub needs_planning: bool,
    /// Whether the task requires research or knowledge retrieval.
    pub needs_research: bool,
    /// Whether the task requires conversation or dialogue.
    pub needs_conversation: bool,
    /// Whether the task requires tool or function calling.
    pub needs_tool_use: bool,
    /// Detected output formats.
    pub output_formats: Vec<String>,
}

/// Responsible for inferring execution requirements from a PromptProfile.
pub trait RequirementInferer {
    /// Infers what the task requires to be completed successfully.
    fn infer(&self, profile: &PromptProfile) -> Result<TaskRequirements, PrismError>;
}

/// A default inferer that uses deterministic heuristics from the prompt profile.
pub struct DefaultRequirementInferer;

impl RequirementInferer for DefaultRequirementInferer {
    fn infer(&self, profile: &PromptProfile) -> Result<TaskRequirements, PrismError> {
        let intrinsic = &profile.intrinsic;
        let derived = &profile.derived;
        let lower = intrinsic.text.to_lowercase();

        let needs_code_generation = derived.task_category == "coding"
            || !intrinsic.languages.is_empty()
            || intrinsic.modality == "code";

        let needs_structured_output = intrinsic.output_format.is_some()
            || lower.contains("json")
            || lower.contains("xml")
            || lower.contains("yaml")
            || lower.contains("csv");

        let needs_reasoning = derived.reasoning_depth == "deep"
            || lower.contains("explain")
            || lower.contains("analyze")
            || lower.contains("compare")
            || lower.contains("why");

        let needs_translation = derived.task_category == "translation"
            || lower.contains("translate")
            || lower.contains(" language");

        let needs_writing = derived.task_category == "creative writing"
            || lower.contains("story")
            || lower.contains("essay")
            || lower.contains("write about");

        let needs_planning = lower.contains("plan")
            || lower.contains("strategy")
            || lower.contains("roadmap")
            || derived.complexity == "high";

        let needs_research = lower.contains("research")
            || lower.contains("search")
            || lower.contains("find")
            || lower.contains("investigate");

        let needs_conversation = derived.task_category == "general"
            && !needs_code_generation
            && !needs_reasoning;

        let needs_tool_use = lower.contains("tool")
            || lower.contains("api call")
            || lower.contains("function")
            || lower.contains("execute");

        let output_formats = intrinsic.output_format.clone().into_iter().collect();

        Ok(TaskRequirements {
            needs_code_generation,
            needs_structured_output,
            needs_reasoning,
            needs_translation,
            needs_writing,
            needs_planning,
            needs_research,
            needs_conversation,
            needs_tool_use,
            output_formats,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::intrinsic::{DefaultIntrinsicExtractor, IntrinsicExtractor};
    use crate::pipeline::derived::{DefaultDerivedAnalyzer, DerivedAnalyzer};
    use prism_core::types::Prompt;

    fn analyze(text: &str) -> PromptProfile {
        let extractor = DefaultIntrinsicExtractor;
        let analyzer = DefaultDerivedAnalyzer;
        let prompt = Prompt { text: text.into() };
        let intrinsic = extractor.extract(&prompt).unwrap();
        let derived = analyzer.analyze(&intrinsic).unwrap();
        PromptProfile { intrinsic, derived }
    }

    #[test]
    fn requirement_infers_coding_from_python_prompt() {
        let profile = analyze("Write a Python function");
        let inferer = DefaultRequirementInferer;
        let reqs = inferer.infer(&profile).unwrap();
        assert!(reqs.needs_code_generation);
    }

    #[test]
    fn requirement_infers_reasoning_from_explain_prompt() {
        let profile = analyze("Explain quantum entanglement");
        let inferer = DefaultRequirementInferer;
        let reqs = inferer.infer(&profile).unwrap();
        assert!(reqs.needs_reasoning);
    }

    #[test]
    fn requirement_infers_translation() {
        let profile = analyze("Translate this to French");
        let inferer = DefaultRequirementInferer;
        let reqs = inferer.infer(&profile).unwrap();
        assert!(reqs.needs_translation);
    }

    #[test]
    fn requirement_infers_structured_output() {
        let profile = analyze("Return the result as JSON");
        let inferer = DefaultRequirementInferer;
        let reqs = inferer.infer(&profile).unwrap();
        assert!(reqs.needs_structured_output);
    }

    #[test]
    fn requirement_infers_planning_for_complex_tasks() {
        let profile = analyze("Plan a multi-step deployment strategy");
        let inferer = DefaultRequirementInferer;
        let reqs = inferer.infer(&profile).unwrap();
        assert!(reqs.needs_planning);
    }
}
