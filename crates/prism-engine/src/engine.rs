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
