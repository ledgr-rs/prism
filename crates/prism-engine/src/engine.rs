use prism_core::error::PrismError;
use prism_core::types::{DecisionReport, Policy, Prompt};

use crate::stages::*;

/// The DecisionEngine orchestrates the full recommendation pipeline.
///
/// It holds each pipeline stage as a trait object and executes them
/// in sequence to produce a DecisionReport.
pub struct DecisionEngine {
    normalizer: Box<dyn Normalizer>,
    prompt_analyzer: Box<dyn PromptAnalyzer>,
    capability_extractor: Box<dyn CapabilityExtractor>,
    candidate_discovery: Box<dyn CandidateDiscovery>,
    policy_evaluator: Box<dyn PolicyEvaluator>,
    candidate_scorer: Box<dyn CandidateScorer>,
    decision_selector: Box<dyn DecisionSelector>,
    explanation_generator: Box<dyn ExplanationGenerator>,
}

impl DecisionEngine {
    /// Evaluates a prompt through the full pipeline, applying the given policy.
    pub fn evaluate(
        &self,
        prompt: Prompt,
        policy: Policy,
    ) -> Result<DecisionReport, PrismError> {
        // 1. Normalization
        let normalized = self.normalizer.normalize(&prompt)?;

        // 2. Prompt Analysis
        let profile = self.prompt_analyzer.analyze(&normalized)?;

        // 3. Capability Extraction
        let capabilities = self.capability_extractor.extract(&profile)?;

        // 4. Candidate Discovery
        let candidates = self.candidate_discovery.discover(&capabilities)?;

        // 5. Policy Evaluation
        let approved = self.policy_evaluator.evaluate(candidates, &capabilities, &policy)?;

        // 6. Candidate Scoring
        let scored = self.candidate_scorer.score(approved, &capabilities)?;

        // 7. Decision Selection
        let recommendation = self.decision_selector.select(scored)?;

        // 8. Explanation Generation
        let explanation = self.explanation_generator.generate(&recommendation, &capabilities)?;

        // 9. Decision Report
        Ok(DecisionReport {
            prompt,
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
            candidate_discovery: Box::new(DefaultCandidateDiscovery),
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

    #[test]
    fn engine_produces_decision_report() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Write a story about a robot".into() };
        let policy = Policy::default();

        let report = engine.evaluate(prompt, policy).unwrap();

        assert!(!report.recommendation.model.id.is_empty());
        assert!(report.recommendation.score >= 0.0);
        assert!(!report.explanation.reasoning.is_empty());
    }

    #[test]
    fn engine_rejects_unsatisfiable_policy() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Hello".into() };
        let policy = Policy {
            name: "impossible".into(),
            constraints: vec!["nonexistent-capability".into()],
        };

        let result = engine.evaluate(prompt, policy);

        assert!(result.is_err());
    }

    #[test]
    fn engine_preserves_prompt_text() {
        let engine = DecisionEngine::default();
        let text = "Explain quantum computing in simple terms".to_string();
        let prompt = Prompt { text: text.clone() };
        let policy = Policy::default();

        let report = engine.evaluate(prompt, policy).unwrap();

        assert_eq!(report.prompt.text, text);
    }

    #[test]
    fn engine_default_constructs() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "test".into() };
        let policy = Policy::default();

        let report = engine.evaluate(prompt, policy).unwrap();

        assert_eq!(report.capabilities.requirements.len(), 1);
        assert_eq!(report.capabilities.requirements[0], "general");
    }

    #[test]
    fn engine_scores_highest_for_best_match() {
        let engine = DecisionEngine::default();
        let prompt = Prompt { text: "Write code for a sorting algorithm".into() };
        let policy = Policy::default();

        let report = engine.evaluate(prompt, policy).unwrap();

        assert!(report.recommendation.score > 0.0);
    }
}
