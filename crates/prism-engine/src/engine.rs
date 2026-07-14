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
    intrinsic_extractor: Box<dyn IntrinsicExtractor>,
    derived_analyzer: Box<dyn DerivedAnalyzer>,
    requirement_inferer: Box<dyn RequirementInferer>,
    capability_mapper: Box<dyn CapabilityMapper>,
    capability_prioritizer: Box<dyn CapabilityPrioritizer>,
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

        // 2. Intrinsic Extraction (observable facts)
        let intrinsic = self.intrinsic_extractor.extract(&normalized)?;

        // 3. Derived Analysis (inferred conclusions)
        let derived = self.derived_analyzer.analyze(&intrinsic)?;

        // 4. Construct PromptProfile
        let profile = PromptProfile { intrinsic, derived };

        // 5. Requirement Inference
        let requirements = self.requirement_inferer.infer(&profile)?;

        // 6. Capability Mapping
        let capabilities = self.capability_mapper.map(&requirements)?;

        // 7. Capability Prioritization
        let capability_requirements = self.capability_prioritizer.prioritize(capabilities)?;

        let capability_profile = prism_core::types::CapabilityProfile {
            requirements: capability_requirements,
        };

        // 8. Candidate Filtering (models supplied by caller)
        let candidates = self
            .candidate_filtering
            .filter(available_models.to_vec(), &capability_profile)?;

        // 9. Policy Evaluation
        let approved = self
            .policy_evaluator
            .evaluate(candidates, &capability_profile, policy)?;

        // 10. Candidate Scoring
        let scored = self.candidate_scorer.score(approved, &capability_profile)?;

        // 11. Decision Selection
        let recommendation = self.decision_selector.select(scored.clone())?;

        // 12. Explanation Generation
        let explanation = self.explanation_generator.generate(
            &recommendation,
            &scored,
            &capability_profile,
            policy,
        )?;

        // 13. Decision Report
        Ok(DecisionReport {
            prompt: prompt.clone(),
            capabilities: capability_profile,
            recommendation,
            explanation,
        })
    }
}

impl Default for DecisionEngine {
    fn default() -> Self {
        Self {
            normalizer: Box::new(DefaultNormalizer),
            intrinsic_extractor: Box::new(DefaultIntrinsicExtractor),
            derived_analyzer: Box::new(DefaultDerivedAnalyzer),
            requirement_inferer: Box::new(DefaultRequirementInferer),
            capability_mapper: Box::new(DefaultCapabilityMapper),
            capability_prioritizer: Box::new(DefaultCapabilityPrioritizer),
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
    use prism_core::types::{
        Capability, CapabilitySupport, ModelIdentity, Policy, PolicyRule, SupportLevel,
    };

    fn capability_support(id: &str, caps: &[&str]) -> ModelProfile {
        let cap_list: Vec<CapabilitySupport> = caps
            .iter()
            .filter_map(|name| {
                // Map string names to Capability enum variants for test scenarios.
                let cap = match *name {
                    "code generation" => Capability::CodeGeneration,
                    "logical reasoning" => Capability::LogicalReasoning,
                    "writing" => Capability::Writing,
                    "general" => Capability::GeneralKnowledge,
                    "translation" => Capability::Translation,
                    _ => return None,
                };
                Some(CapabilitySupport {
                    capability: cap,
                    support_level: SupportLevel::Full,
                    confidence: 1.0,
                })
            })
            .collect();
        ModelProfile {
            identity: ModelIdentity {
                id: id.into(),
                name: id.into(),
                provider: "test".into(),
                ..Default::default()
            },
            capabilities: cap_list,
            ..Default::default()
        }
    }

    fn sample_models() -> Vec<ModelProfile> {
        vec![
            capability_support("model-a", &["code generation", "logical reasoning"]),
            capability_support("model-b", &["writing", "general"]),
            capability_support("model-c", &["code generation", "translation", "logical reasoning"]),
        ]
    }

    #[test]
    fn engine_produces_decision_report() {
        let engine = DecisionEngine::default();
        let prompt = Prompt {
            text: "Write a story about a robot".into(),
        };
        let models = sample_models();
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        assert!(!report.recommendation.model.identity.id.is_empty());
        assert!(report.recommendation.score >= 0.0);
        assert!(!report.explanation.summary.is_empty());
    }

    #[test]
    fn engine_rejects_empty_models() {
        let engine = DecisionEngine::default();
        let prompt = Prompt {
            text: "Hello".into(),
        };
        let models = vec![];
        let policy = Policy::default();

        let result = engine.evaluate(&prompt, &models, &policy);

        assert!(result.is_err());
    }

    #[test]
    fn engine_rejects_unsatisfiable_policy() {
        let engine = DecisionEngine::default();
        let prompt = Prompt {
            text: "Hello".into(),
        };
        let models = sample_models();
        let policy = Policy {
            name: "impossible".into(),
            rules: vec![PolicyRule::PreferredProviders(vec!["nonexistent".into()])],
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
        let prompt = Prompt {
            text: "Implement a sorting algorithm in Python".into(),
        };
        let models = sample_models();
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        assert_eq!(report.recommendation.model.identity.id, "model-c");
        assert!(report.recommendation.score > 0.0);
    }

    #[test]
    fn engine_explanation_mentions_selected_model() {
        let engine = DecisionEngine::default();
        let prompt = Prompt {
            text: "Translate this document".into(),
        };
        let models = sample_models();
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        assert!(report
            .explanation
            .summary
            .contains(&report.recommendation.model.identity.id));
    }

    #[test]
    fn engine_uses_caller_supplied_models_only() {
        let engine = DecisionEngine::default();
        let prompt = Prompt {
            text: "Implement a sorting algorithm in Python".into(),
        };
        let models = vec![capability_support("only-model", &["code generation"])];
        let policy = Policy::default();

        let report = engine.evaluate(&prompt, &models, &policy).unwrap();

        assert_eq!(report.recommendation.model.identity.id, "only-model");
    }
}
