use prism_core::error::PrismError;
use prism_core::types::{
    CandidateScore, CapabilityMatch, CapabilityProfile, Evidence, Explanation, MatchStatus,
    ModelProfile, Policy, PolicyDecision, PolicyRule, Recommendation, RejectedAlternative,
    SupportLevel,
};

/// Responsible for generating a structured explanation for a recommendation.
pub trait ExplanationGenerator {
    /// Produces a structured explanation that justifies the given recommendation.
    fn generate(
        &self,
        recommendation: &Recommendation,
        scored_candidates: &[(ModelProfile, CandidateScore)],
        capabilities: &CapabilityProfile,
        policy: &Policy,
    ) -> Result<Explanation, PrismError>;
}

/// A default generator that produces a deterministic, structured explanation.
pub struct DefaultExplanationGenerator;

impl ExplanationGenerator for DefaultExplanationGenerator {
    fn generate(
        &self,
        recommendation: &Recommendation,
        scored_candidates: &[(ModelProfile, CandidateScore)],
        capabilities: &CapabilityProfile,
        policy: &Policy,
    ) -> Result<Explanation, PrismError> {
        let fulfilled_count = capabilities
            .requirements
            .iter()
            .filter(|req| {
                recommendation
                    .model
                    .supports_capability(&req.capability)
            })
            .count();

        let total = capabilities.requirements.len();

        let summary = format!(
            "Selected '{}' (score: {:.1}) — fulfills {} of {} capability requirements.",
            recommendation.model.identity.id,
            recommendation.score,
            fulfilled_count,
            total
        );

        let evidence: Vec<Evidence> = capabilities
            .requirements
            .iter()
            .map(|req| Evidence {
                detail: format!("'{}' (priority: {:?})", req.capability, req.priority),
                source: req.reason.clone(),
            })
            .collect();

        let capability_matches: Vec<CapabilityMatch> = capabilities
            .requirements
            .iter()
            .map(|req| {
                let cap_str = req.capability.to_string();
                let support = recommendation
                    .model
                    .capabilities
                    .iter()
                    .find(|cs| cs.capability == req.capability)
                    .map(|cs| &cs.support_level);

                let (status, reason) = match support {
                    Some(SupportLevel::Full) | Some(SupportLevel::Native) => (
                        MatchStatus::Fulfilled,
                        "Candidate fully supports this capability",
                    ),
                    Some(SupportLevel::Partial) => (
                        MatchStatus::Partial,
                        "Candidate partially supports this capability",
                    ),
                    Some(SupportLevel::None) | None => (
                        MatchStatus::Unfulfilled,
                        "Candidate does not support this capability",
                    ),
                };

                CapabilityMatch {
                    capability: cap_str,
                    status,
                    reason: reason.into(),
                }
            })
            .collect();

        let policy_decisions: Vec<PolicyDecision> = if policy.rules.is_empty() {
            vec![PolicyDecision {
                policy: policy.name.clone(),
                applied: false,
                reason: "No policy rules were configured.".into(),
            }]
        } else {
            policy
                .rules
                .iter()
                .map(|rule| {
                    let description = describe_policy_rule(rule);
                    PolicyDecision {
                        policy: description,
                        applied: true,
                        reason: "Policy rule was evaluated against all candidates.".into(),
                    }
                })
                .collect()
        };

        let rejected_alternatives: Vec<RejectedAlternative> = scored_candidates
            .iter()
            .filter(|(m, _)| m.identity.id != recommendation.model.identity.id)
            .map(|(m, s)| {
                let delta = recommendation.score - s.final_score;
                RejectedAlternative {
                    model_id: m.identity.id.clone(),
                    score: s.final_score,
                    reason: format!(
                        "Score {:.1} is {:.1} lower than selected model",
                        s.final_score, delta
                    ),
                }
            })
            .collect();

        let confidence = if capabilities.requirements.is_empty() {
            0.0
        } else {
            capabilities
                .requirements
                .iter()
                .map(|r| r.confidence)
                .sum::<f64>()
                / capabilities.requirements.len() as f64
        };

        Ok(Explanation {
            summary,
            evidence,
            capability_matches,
            policy_decisions,
            rejected_alternatives,
            confidence,
        })
    }
}

fn describe_policy_rule(rule: &PolicyRule) -> String {
    match rule {
        PolicyRule::MaxBudget(limit) => format!("max budget: {limit}"),
        PolicyRule::MaxLatency(limit) => format!("max latency: {limit}ms"),
        PolicyRule::PrivacyLocalOnly => "privacy: local only".into(),
        PolicyRule::PreferredProviders(providers) => {
            format!("preferred providers: {}", providers.join(", "))
        }
        PolicyRule::ForbiddenProviders(providers) => {
            format!("forbidden providers: {}", providers.join(", "))
        }
        PolicyRule::RequiredRegion(region) => format!("required region: {region}"),
        PolicyRule::MinConfidence(min) => format!("min confidence: {min}"),
    }
}
