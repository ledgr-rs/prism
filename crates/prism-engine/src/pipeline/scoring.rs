use prism_core::error::PrismError;
use prism_core::types::{CandidateScore, CapabilityProfile, ModelProfile, Priority, SupportLevel};

/// Factor applied per extra capability a model supports that is not in the requirements.
const EXTRA_CAPABILITY_BONUS: f64 = 0.05;

/// Returns a multiplier based on how well a model supports a capability.
fn support_level_multiplier(level: &SupportLevel) -> f64 {
    match level {
        SupportLevel::Native => 1.0,
        SupportLevel::Full => 0.9,
        SupportLevel::Partial => 0.5,
        SupportLevel::None => 0.0,
    }
}

/// Computes the priority-dependent penalty factor for a missing capability.
fn missing_penalty_factor(priority: &Priority) -> f64 {
    match priority {
        Priority::Required => 2.0,
        Priority::Preferred => 0.5,
        Priority::Optional => 0.0,
    }
}

/// Responsible for scoring candidate models using explainable evidence.
pub trait CandidateScorer {
    /// Scores each candidate model and returns a list of (model, score) pairs.
    fn score(
        &self,
        candidates: Vec<ModelProfile>,
        capabilities: &CapabilityProfile,
    ) -> Result<Vec<(ModelProfile, CandidateScore)>, PrismError>;
}

/// A default scorer that uses deterministic, evidence-based heuristics.
///
/// Scoring formula:
/// - For each required capability the model supports: contribution = weight * support_level_multiplier
/// - For each required capability the model lacks: penalty = weight * 2.0
/// - For each preferred capability the model supports: contribution = weight * support_level_multiplier
/// - For each preferred capability the model lacks: penalty = weight * 0.5
/// - For each optional capability the model supports: contribution = weight * support_level_multiplier
/// - Missing optional capabilities incur no penalty
/// - Extra capabilities (not in requirements) give a small bonus
///
/// Every scoring decision generates explainable evidence. No randomness.
pub struct DefaultCandidateScorer;

impl CandidateScorer for DefaultCandidateScorer {
    fn score(
        &self,
        candidates: Vec<ModelProfile>,
        capabilities: &CapabilityProfile,
    ) -> Result<Vec<(ModelProfile, CandidateScore)>, PrismError> {
        let scored: Vec<(ModelProfile, CandidateScore)> = candidates
            .into_iter()
            .map(|model| score_candidate(&model, capabilities))
            .collect();

        Ok(scored)
    }
}

fn score_candidate(model: &ModelProfile, capabilities: &CapabilityProfile) -> (ModelProfile, CandidateScore) {
    let mut total_score = 0.0;
    let mut evidence: Vec<String> = Vec::new();
    let mut bonuses: Vec<String> = Vec::new();
    let mut penalties: Vec<String> = Vec::new();
    let mut matched_confidences: Vec<f64> = Vec::new();

    for req in &capabilities.requirements {
        let cap_name = req.capability.to_string();
        let is_supported = model.supports_capability(&req.capability);

        if is_supported {
            let sl_mult = model.capabilities
                .iter()
                .find(|cs| cs.capability == req.capability)
                .map(|cs| support_level_multiplier(&cs.support_level))
                .unwrap_or(0.0);

            let contribution = req.weight * sl_mult;
            total_score += contribution;
            matched_confidences.push(req.confidence);

            let priority_label = format!("{:?}", req.priority).to_lowercase();
            evidence.push(format!(
                "Capability '{}' is {} and is supported by the model (contribution: {:.4})",
                cap_name, priority_label, contribution
            ));

            if req.priority == Priority::Required {
                bonuses.push(format!("Required capability '{}' matched", cap_name));
            }
        } else {
            let penalty_factor = missing_penalty_factor(&req.priority);
            if penalty_factor > 0.0 {
                let penalty = req.weight * penalty_factor;
                total_score -= penalty;
                penalties.push(format!(
                    "Missing {} capability '{}' (penalty: {:.4})",
                    format!("{:?}", req.priority).to_lowercase(),
                    cap_name,
                    penalty
                ));
            }

            let priority_label = format!("{:?}", req.priority).to_lowercase();
            evidence.push(format!(
                "Capability '{}' is {} but is NOT supported by the model",
                cap_name, priority_label
            ));
        }
    }

    let required_caps: Vec<&prism_core::types::Capability> = capabilities
        .requirements
        .iter()
        .map(|r| &r.capability)
        .collect();

    for cs in &model.capabilities {
        if !required_caps.contains(&&cs.capability) && cs.support_level != SupportLevel::None {
            total_score += EXTRA_CAPABILITY_BONUS;
            evidence.push(format!(
                "Extra capability '{}' provides additional value (bonus: {:.2})",
                cs.capability, EXTRA_CAPABILITY_BONUS
            ));
            bonuses.push(format!("Extra capability '{}' supported", cs.capability));
        }
    }

    let confidence = if matched_confidences.is_empty() {
        0.0
    } else {
        matched_confidences.iter().sum::<f64>() / matched_confidences.len() as f64
    };

    let candidate_score = CandidateScore {
        final_score: total_score,
        confidence,
        evidence,
        bonuses,
        penalties,
    };

    (model.clone(), candidate_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use prism_core::types::{
        Capability, CapabilityRequirement, CapabilitySupport, ModelIdentity, Priority,
        SupportLevel,
    };

    fn model_with_capabilities(id: &str, caps: Vec<(Capability, SupportLevel)>) -> ModelProfile {
        ModelProfile {
            identity: ModelIdentity {
                id: id.into(),
                name: id.into(),
                provider: "test".into(),
                version: None,
                family: None,
            },
            capabilities: caps
                .into_iter()
                .map(|(cap, level)| CapabilitySupport {
                    capability: cap,
                    support_level: level,
                    confidence: 1.0,
                })
                .collect(),
            ..Default::default()
        }
    }

    fn capability_profile(requirements: Vec<(Capability, Priority, f64)>) -> CapabilityProfile {
        CapabilityProfile {
            requirements: requirements
                .into_iter()
                .map(|(cap, priority, weight)| CapabilityRequirement {
                    capability: cap.clone(),
                    priority,
                    weight,
                    confidence: 0.9,
                    reason: format!("test requirement for {}", cap),
                })
                .collect(),
        }
    }

    #[test]
    fn matching_required_capability_yields_positive_score() {
        let model = model_with_capabilities(
            "model-a",
            vec![(Capability::CodeGeneration, SupportLevel::Full)],
        );
        let profile = capability_profile(vec![(
            Capability::CodeGeneration,
            Priority::Required,
            1.0,
        )]);

        let (_, score) = score_candidate(&model, &profile);

        assert!(score.final_score > 0.0);
        assert!(!score.evidence.is_empty());
        assert!(!score.bonuses.is_empty());
        assert!(score.penalties.is_empty());
    }

    #[test]
    fn missing_required_capability_applies_penalty() {
        let model = model_with_capabilities("model-b", vec![]);
        let profile = capability_profile(vec![(
            Capability::CodeGeneration,
            Priority::Required,
            1.0,
        )]);

        let (_, score) = score_candidate(&model, &profile);

        assert!(score.final_score < 0.0);
        assert!(!score.evidence.is_empty());
        assert!(score.bonuses.is_empty());
        assert!(!score.penalties.is_empty());
    }

    #[test]
    fn preferred_capability_weaker_than_required() {
        let model_preferred = model_with_capabilities(
            "pref",
            vec![(Capability::Writing, SupportLevel::Full)],
        );
        let model_required = model_with_capabilities(
            "req",
            vec![(Capability::CodeGeneration, SupportLevel::Full)],
        );
        let profile = capability_profile(vec![
            (
                Capability::CodeGeneration,
                Priority::Required,
                1.0,
            ),
            (Capability::Writing, Priority::Preferred, 0.7),
        ]);

        let (_, score_req) = score_candidate(&model_required, &profile);
        let (_, score_pref) = score_candidate(&model_preferred, &profile);

        // Required-capability model should score higher even with same weight
        // because the preferred model gets a penalty for missing required cap
        assert!(score_req.final_score > score_pref.final_score);
    }

    #[test]
    fn extra_capabilities_add_bonus() {
        let model = model_with_capabilities(
            "extra",
            vec![
                (Capability::CodeGeneration, SupportLevel::Full),
                (Capability::Translation, SupportLevel::Full),
            ],
        );
        let profile = capability_profile(vec![(
            Capability::CodeGeneration,
            Priority::Required,
            1.0,
        )]);

        let (_, score) = score_candidate(&model, &profile);

        // Extra Translation capability should add bonus
        assert!(
            score.final_score > 0.9,
            "Expected score > 0.9 with extra capability bonus, got {}",
            score.final_score
        );
        assert!(score.bonuses.iter().any(|b| b.contains("translation")));
    }

    #[test]
    fn support_level_affects_score() {
        let model_full = model_with_capabilities(
            "full",
            vec![(Capability::CodeGeneration, SupportLevel::Full)],
        );
        let model_partial = model_with_capabilities(
            "partial",
            vec![(Capability::CodeGeneration, SupportLevel::Partial)],
        );
        let profile = capability_profile(vec![(
            Capability::CodeGeneration,
            Priority::Required,
            1.0,
        )]);

        let (_, score_full) = score_candidate(&model_full, &profile);
        let (_, score_partial) = score_candidate(&model_partial, &profile);

        assert!(score_full.final_score > score_partial.final_score);
    }

    #[test]
    fn missing_optional_capability_no_penalty() {
        let model = model_with_capabilities(
            "basic",
            vec![(Capability::CodeGeneration, SupportLevel::Full)],
        );
        let profile = capability_profile(vec![
            (Capability::CodeGeneration, Priority::Required, 1.0),
            (Capability::Vision, Priority::Optional, 0.5),
        ]);

        let (_, score) = score_candidate(&model, &profile);

        // Score should be positive (no penalty for missing optional)
        assert!(score.final_score > 0.0);
        assert!(score.penalties.is_empty());
    }

    #[test]
    fn confidence_is_average_of_matched_requirements() {
        let model = model_with_capabilities(
            "conf",
            vec![
                (Capability::CodeGeneration, SupportLevel::Full),
                (Capability::Writing, SupportLevel::Full),
            ],
        );
        let profile = CapabilityProfile {
            requirements: vec![
                CapabilityRequirement {
                    capability: Capability::CodeGeneration,
                    priority: Priority::Required,
                    weight: 1.0,
                    confidence: 0.9,
                    reason: "test".into(),
                },
                CapabilityRequirement {
                    capability: Capability::Writing,
                    priority: Priority::Preferred,
                    weight: 0.7,
                    confidence: 0.8,
                    reason: "test".into(),
                },
            ],
        };

        let (_, score) = score_candidate(&model, &profile);

        assert!((score.confidence - 0.85).abs() < 0.001);
    }

    #[test]
    fn deterministic_scoring() {
        let model = model_with_capabilities(
            "same",
            vec![(Capability::CodeGeneration, SupportLevel::Full)],
        );
        let profile = capability_profile(vec![(
            Capability::CodeGeneration,
            Priority::Required,
            1.0,
        )]);

        let (_, score1) = score_candidate(&model, &profile);
        let (_, score2) = score_candidate(&model, &profile);

        assert_eq!(score1.final_score, score2.final_score);
        assert_eq!(score1.evidence, score2.evidence);
        assert_eq!(score1.bonuses, score2.bonuses);
        assert_eq!(score1.penalties, score2.penalties);
    }
}
