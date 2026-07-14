use prism_core::error::PrismError;
use prism_core::types::{CapabilityProfile, ModelProfile, Policy, PolicyRule};

/// Responsible for applying policy constraints to filter candidate models.
pub trait PolicyEvaluator {
    /// Filters the given candidates using the provided policy and capability requirements.
    fn evaluate(
        &self,
        candidates: Vec<ModelProfile>,
        capabilities: &CapabilityProfile,
        policy: &Policy,
    ) -> Result<Vec<ModelProfile>, PrismError>;
}

/// A default evaluator that composes individual policy rules.
///
/// Each rule is evaluated independently against every candidate.
/// All rules must be satisfied for a candidate to pass.
/// An empty rule set permits all candidates.
pub struct DefaultPolicyEvaluator;

impl PolicyEvaluator for DefaultPolicyEvaluator {
    fn evaluate(
        &self,
        candidates: Vec<ModelProfile>,
        _capabilities: &CapabilityProfile,
        policy: &Policy,
    ) -> Result<Vec<ModelProfile>, PrismError> {
        if policy.rules.is_empty() {
            return Ok(candidates);
        }

        let filtered: Vec<ModelProfile> = candidates
            .into_iter()
            .filter(|model| policy.rules.iter().all(|rule| rule_satisfies(model, rule)))
            .collect();

        if filtered.is_empty() {
            return Err(PrismError::MissingInformation(
                "No candidates satisfy the active policy".to_string(),
            ));
        }

        Ok(filtered)
    }
}

fn rule_satisfies(model: &ModelProfile, rule: &PolicyRule) -> bool {
    match rule {
        PolicyRule::MaxBudget(limit) => model
            .operational_characteristics
            .cost
            .as_ref()
            .map_or(true, |c| c.per_input_token <= *limit),
        PolicyRule::MaxLatency(limit) => model
            .operational_characteristics
            .latency
            .as_ref()
            .map_or(true, |l| l.p50_ms <= *limit as f64),
        PolicyRule::PrivacyLocalOnly => {
            matches!(model.operational_characteristics.locality, prism_core::types::Locality::Local)
        }
        PolicyRule::PreferredProviders(providers) => {
            providers.contains(&model.identity.provider)
        }
        PolicyRule::ForbiddenProviders(providers) => {
            !providers.contains(&model.identity.provider)
        }
        PolicyRule::RequiredRegion(region) => model
            .metadata
            .tags
            .iter()
            .any(|t| t.contains(region.as_str())),
        PolicyRule::MinConfidence(min) => model
            .capabilities
            .iter()
            .map(|cs| cs.confidence)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map_or(true, |c| c >= *min),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prism_core::types::{
        Capability, CapabilitySupport, CostProfile, LatencyProfile, Locality, ModelIdentity,
        OperationalCharacteristics, SupportLevel,
    };

    fn model(id: &str, provider: &str, cost: Option<f64>, latency: Option<f64>, locality: Locality, tags: Vec<String>, caps: Vec<(Capability, f64)>) -> ModelProfile {
        ModelProfile {
            identity: ModelIdentity {
                id: id.into(),
                name: id.into(),
                provider: provider.into(),
                version: None,
                family: None,
            },
            operational_characteristics: OperationalCharacteristics {
                cost: cost.map(|c| CostProfile {
                    per_input_token: c,
                    per_output_token: 0.0,
                    currency: "USD".into(),
                }),
                latency: latency.map(|l| LatencyProfile {
                    p50_ms: l,
                    p95_ms: l * 2.0,
                }),
                locality,
                ..Default::default()
            },
            capabilities: caps
                .into_iter()
                .map(|(cap, conf)| CapabilitySupport {
                    capability: cap,
                    support_level: SupportLevel::Full,
                    confidence: conf,
                })
                .collect(),
            metadata: prism_core::types::ModelMetadata {
                tags,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn sample_models() -> Vec<ModelProfile> {
        vec![
            model(
                "cheap-local", "ollama", Some(0.0), Some(50.0),
                Locality::Local, vec![],
                vec![(Capability::General, 0.5)],
            ),
            model(
                "cloud-fast", "openai", Some(0.01), Some(200.0),
                Locality::Cloud, vec!["us-east".into()],
                vec![(Capability::General, 0.95)],
            ),
            model(
                "cloud-cheap", "anthropic", Some(0.002), Some(500.0),
                Locality::Cloud, vec!["eu-west".into()],
                vec![(Capability::General, 0.90)],
            ),
            model(
                "expensive-slow", "openai", Some(0.1), Some(2000.0),
                Locality::Cloud, vec!["us-east".into()],
                vec![(Capability::General, 0.99)],
            ),
        ]
    }

    fn empty_capabilities() -> CapabilityProfile {
        CapabilityProfile { requirements: vec![] }
    }

    #[test]
    fn empty_policy_passes_all_models() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy::default();
        let result = evaluator.evaluate(models.clone(), &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), models.len());
    }

    #[test]
    fn budget_filters_expensive_models() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "budget".into(),
            rules: vec![PolicyRule::MaxBudget(0.005)],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|m| m.identity.id != "cloud-fast"));
        assert!(result.iter().all(|m| m.identity.id != "expensive-slow"));
    }

    #[test]
    fn latency_filters_slow_models() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "latency".into(),
            rules: vec![PolicyRule::MaxLatency(300)],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|m| m.identity.id == "cheap-local"));
        assert!(result.iter().any(|m| m.identity.id == "cloud-fast"));
    }

    #[test]
    fn preferred_providers_allowlist() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "prefer-anthropic".into(),
            rules: vec![PolicyRule::PreferredProviders(vec!["anthropic".into()])],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].identity.id, "cloud-cheap");
    }

    #[test]
    fn forbidden_providers_denylist() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "no-openai".into(),
            rules: vec![PolicyRule::ForbiddenProviders(vec!["openai".into()])],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|m| m.identity.provider != "openai"));
    }

    #[test]
    fn privacy_local_only() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "local-only".into(),
            rules: vec![PolicyRule::PrivacyLocalOnly],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].identity.id, "cheap-local");
    }

    #[test]
    fn required_region_filters() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "eu-only".into(),
            rules: vec![PolicyRule::RequiredRegion("eu-west".into())],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].identity.id, "cloud-cheap");
    }

    #[test]
    fn minimum_confidence_filters() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "high-confidence".into(),
            rules: vec![PolicyRule::MinConfidence(0.95)],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|m| {
            m.capabilities.iter().any(|cs| cs.confidence >= 0.95)
        }));
    }

    #[test]
    fn multiple_rules_compose_with_and() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "openai-budget".into(),
            rules: vec![
                PolicyRule::PreferredProviders(vec!["openai".into()]),
                PolicyRule::MaxBudget(0.05),
            ],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].identity.id, "cloud-fast");
    }

    #[test]
    fn no_models_satisfy_policy_returns_error() {
        let evaluator = DefaultPolicyEvaluator;
        let models = sample_models();
        let policy = Policy {
            name: "impossible".into(),
            rules: vec![PolicyRule::PreferredProviders(vec!["nonexistent".into()])],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy);
        assert!(result.is_err());
    }

    #[test]
    fn model_without_metadata_passes_optional_checks() {
        let evaluator = DefaultPolicyEvaluator;
        let models = vec![ModelProfile::default()];
        let policy = Policy {
            name: "lenient".into(),
            rules: vec![
                PolicyRule::MaxBudget(0.0),
                PolicyRule::MaxLatency(1),
                PolicyRule::MinConfidence(0.99),
            ],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn model_without_metadata_fails_assertive_checks() {
        let evaluator = DefaultPolicyEvaluator;
        let models = vec![ModelProfile::default()];
        let policy = Policy {
            name: "strict".into(),
            rules: vec![PolicyRule::PreferredProviders(vec!["openai".into()])],
        };
        let result = evaluator.evaluate(models, &empty_capabilities(), &policy);
        assert!(result.is_err());
    }
}
