use prism_core::error::PrismError;
use prism_core::types::{DecisionReport, ModelProfile, Policy, Prompt};

use crate::stages::*;

/// The DecisionEngine orchestrates the full recommendation pipeline.
///
/// It holds each pipeline stage as a trait object and executes them
/// in sequence to produce a DecisionReport.
///
/// The engine never owns or discovers models — the caller supplies them.
pub struct DecisionEngine {
    normalizer: Box<dyn Normalizer>,
    prompt_analyzer: Box<dyn PromptAnalyzer>,
    capability_extractor: Box<dyn CapabilityExtractor>,
    candidate_filtering: Box<dyn CandidateFiltering>,
    policy_evaluator: Box<dyn PolicyEvaluator>,
    candidate_scorer: Box<dyn CandidateScorer>,
    decision_selector: Box<dyn DecisionSelector>,
    explanation_generator: Box<dyn ExplanationGenerator>,
}

impl DecisionEngine {
    /// Evaluates a prompt through the full pipeline.
    ///
    /// The caller supplies the available models — the engine never discovers them.
    pub fn evaluate(
        &self,
        prompt: &Prompt,
        available_models: &[ModelProfile],
        policy: &Policy,
    ) -> Result<DecisionReport, PrismError> {
        // 1. Normalization
        let normalized = self.normalizer.normalize(prompt)?;

        // 2. Prompt Analysis
        let profile = self.prompt_analyzer.analyze(&normalized)?;

        // 3. Capability Extraction
        let capabilities = self.capability_extractor.extract(&profile)?;

        // 4. Candidate Filtering (models supplied by caller)
        let candidates = self.candidate_filtering.filter(available_models.to_vec(), &capabilities)?;

        // 5. Policy Evaluation
        let approved = self.policy_evaluator.evaluate(candidates, &capabilities, policy)?;

        // 6. Candidate Scoring
        let scored = self.candidate_scorer.score(approved, &capabilities)?;

        // 7. Decision Selection
        let recommendation = self.decision_selector.select(scored)?;

        // 8. Explanation Generation
        let explanation = self.explanation_generator.generate(&recommendation, &capabilities)?;

        // 9. Decision Report
        Ok(DecisionReport {
            prompt: prompt.clone(),
            capabilities,
            recommendation,
            explanation,
        })
    }
}

impl Default for DecisionEngine {
    fn default() -> Self {
        Self {
            normalizer: Box::new(DefaultNormalizer),
            prompt_analyzer: Box::new(DefaultPromptAnalyzer),
            capability_extractor: Box::new(DefaultCapabilityExtractor),
            candidate_filtering: Box::new(DefaultCandidateFiltering),
            policy_evaluator: Box::new(DefaultPolicyEvaluator),
            candidate_scorer: Box::new(DefaultCandidateScorer),
            decision_selector: Box::new(DefaultDecisionSelector),
            explanation_generator: Box::new(DefaultExplanationGenerator),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prism_core::types::Policy;

    fn sample_models() -> Vec<ModelProfile> {
        vec![
            ModelProfile {
                id: "model-a".into(),
                capabilities: vec!["coding".into(), "reasoning".into()],
            },
            ModelProfile {
                id: "model-b".into(),
                capabilities: vec!["creative writing".into(), "general".into()],
            },
            ModelProfile {
                id: "model-c".into(),
                capabilities: vec!["coding".into(), "translation".into(), "reasoning".into()],
            },
        ]
    }

    #[test]
    fn engine_produces_decision_report() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Write a story about a robot".into() };
        let models = sample_models();
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        assert!(!report.recommendation.model.id.is_empty());
        assert!(report.recommendation.score >= 0.0);
        assert!(!report.explanation.reasoning.is_empty());
    }

    #[test]
    fn engine_rejects_empty_models() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Hello".into() };
        let models = vec![];
        let policy = Policy::default();

        let result = engine.evaluate(&prompt, &models, &policy);

        assert!(result.is_err());
    }

    #[test]
    fn engine_rejects_unsatisfiable_policy() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Hello".into() };
        let models = sample_models();
        let policy = Policy {
            name: "impossible".into(),
            constraints: vec!["nonexistent-capability".into()],
        };

        let result = engine.evaluate(&prompt, &models, &policy);

        assert!(result.is_err());
    }

    #[test]
    fn engine_preserves_prompt_text() {
        let engine = DecisionEngine::default();
        let text = "Explain quantum computing in simple terms".to_string();
        let prompt = Prompt { text: text.clone() };
        let models = sample_models();
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        assert_eq!(report.prompt.text, text);
    }

    #[test]
    fn engine_selects_highest_scored_model() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Write code for a sorting algorithm".into() };
        let models = sample_models();
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        // model-c has coding + reasoning, best match for "coding"
        assert_eq!(report.recommendation.model.id, "model-c");
        assert!(report.recommendation.score > 0.0);
    }

    #[test]
    fn engine_explanation_mentions_selected_model() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Translate this document".into() };
        let models = sample_models();
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        assert!(report.explanation.reasoning.contains(&report.recommendation.model.id));
    }

    #[test]
    fn engine_uses_caller_supplied_models_only() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Write code".into() };
        let models = vec![
            ModelProfile {
                id: "only-model".into(),
                capabilities: vec!["coding".into()],
            },
        ];
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        assert_eq!(report.recommendation.model.id, "only-model");
    }
}
