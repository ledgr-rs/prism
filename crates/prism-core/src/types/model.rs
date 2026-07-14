use crate::types::Capability;

/// How well a model supports a given capability.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SupportLevel {
    #[default]
    None,
    Partial,
    Full,
    Native,
}

/// Associates a capability with its support level and confidence.
#[derive(Debug, Clone, PartialEq)]
pub struct CapabilitySupport {
    pub capability: Capability,
    pub support_level: SupportLevel,
    pub confidence: f64,
}

/// Immutable identity information for a model.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelIdentity {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub version: Option<String>,
    pub family: Option<String>,
}

impl Default for ModelIdentity {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            provider: String::new(),
            version: None,
            family: None,
        }
    }
}

/// Deployment locality of a model.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Locality {
    Cloud,
    Local,
    Hybrid,
    #[default]
    Unknown,
}

/// Privacy handling level of a model.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum PrivacyLevel {
    #[default]
    None,
    Low,
    Medium,
    High,
}

/// Measured or advertised latency values.
#[derive(Debug, Clone, PartialEq)]
pub struct LatencyProfile {
    pub p50_ms: f64,
    pub p95_ms: f64,
}

impl Default for LatencyProfile {
    fn default() -> Self {
        Self {
            p50_ms: 0.0,
            p95_ms: 0.0,
        }
    }
}

/// Pricing information for a model.
#[derive(Debug, Clone, PartialEq)]
pub struct CostProfile {
    pub per_input_token: f64,
    pub per_output_token: f64,
    pub currency: String,
}

impl Default for CostProfile {
    fn default() -> Self {
        Self {
            per_input_token: 0.0,
            per_output_token: 0.0,
            currency: "USD".into(),
        }
    }
}

/// Operational characteristics that describe how a model behaves.
///
/// These are separate from capabilities — they describe performance,
/// cost, and deployment attributes rather than functional abilities.
#[derive(Debug, Clone, PartialEq)]
pub struct OperationalCharacteristics {
    pub latency: Option<LatencyProfile>,
    pub cost: Option<CostProfile>,
    pub context_window: usize,
    pub streaming: bool,
    pub locality: Locality,
    pub privacy_level: PrivacyLevel,
    pub region: Option<String>,
}

impl Default for OperationalCharacteristics {
    fn default() -> Self {
        Self {
            latency: None,
            cost: None,
            context_window: 0,
            streaming: false,
            locality: Locality::default(),
            privacy_level: PrivacyLevel::default(),
            region: None,
        }
    }
}

/// Hard constraints of a model.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelLimits {
    pub max_context_tokens: usize,
    pub max_output_tokens: usize,
}

impl Default for ModelLimits {
    fn default() -> Self {
        Self {
            max_context_tokens: 0,
            max_output_tokens: 0,
        }
    }
}

/// Additional descriptive information about a model.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModelMetadata {
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub documentation_url: Option<String>,
    pub release_date: Option<String>,
    pub confidence: Option<f64>,
}

/// The canonical representation of a model within Prism.
///
/// Contains only metadata — no routing, scoring, execution, or provider logic.
/// Immutable during evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelProfile {
    pub identity: ModelIdentity,
    pub capabilities: Vec<CapabilitySupport>,
    pub operational_characteristics: OperationalCharacteristics,
    pub limits: ModelLimits,
    pub metadata: ModelMetadata,
}

impl Default for ModelProfile {
    fn default() -> Self {
        Self {
            identity: ModelIdentity::default(),
            capabilities: Vec::new(),
            operational_characteristics: OperationalCharacteristics::default(),
            limits: ModelLimits::default(),
            metadata: ModelMetadata::default(),
        }
    }
}

impl ModelProfile {
    /// Returns true if this model supports the given capability (support level is not None).
    pub fn supports_capability(&self, capability: &Capability) -> bool {
        self.capabilities
            .iter()
            .any(|cs| cs.capability == *capability && cs.support_level != SupportLevel::None)
    }

    /// Returns true if this model supports a capability matching the given display name.
    pub fn supports_capability_by_name(&self, name: &str) -> bool {
        self.capabilities
            .iter()
            .any(|cs| cs.capability.to_string() == name && cs.support_level != SupportLevel::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Capability;

    // -----------------------------------------------------------------------
    // ModelIdentity
    // -----------------------------------------------------------------------

    #[test]
    fn model_identity_defaults() {
        let id = ModelIdentity::default();
        assert!(id.id.is_empty());
        assert!(id.name.is_empty());
        assert!(id.provider.is_empty());
        assert!(id.version.is_none());
        assert!(id.family.is_none());
    }

    #[test]
    fn model_identity_construction() {
        let id = ModelIdentity {
            id: "gpt-4o".into(),
            name: "GPT-4o".into(),
            provider: "openai".into(),
            version: Some("2024-08-06".into()),
            family: Some("gpt-4".into()),
        };
        assert_eq!(id.id, "gpt-4o");
        assert_eq!(id.name, "GPT-4o");
        assert_eq!(id.provider, "openai");
        assert_eq!(id.version, Some("2024-08-06".into()));
        assert_eq!(id.family, Some("gpt-4".into()));
    }

    #[test]
    fn model_identity_version_optional() {
        let id = ModelIdentity {
            id: "claude-3-haiku".into(),
            name: "Claude 3 Haiku".into(),
            provider: "anthropic".into(),
            version: None,
            family: Some("claude-3".into()),
        };
        assert!(id.version.is_none());
    }

    // -----------------------------------------------------------------------
    // SupportLevel
    // -----------------------------------------------------------------------

    #[test]
    fn support_level_default_is_none() {
        let level = SupportLevel::default();
        assert_eq!(level, SupportLevel::None);
    }

    #[test]
    fn support_level_variants() {
        assert_ne!(SupportLevel::None, SupportLevel::Partial);
        assert_ne!(SupportLevel::Partial, SupportLevel::Full);
        assert_ne!(SupportLevel::Full, SupportLevel::Native);
    }

    // -----------------------------------------------------------------------
    // CapabilitySupport
    // -----------------------------------------------------------------------

    #[test]
    fn capability_support_construction() {
        let cs = CapabilitySupport {
            capability: Capability::CodeGeneration,
            support_level: SupportLevel::Full,
            confidence: 0.95,
        };
        assert_eq!(cs.capability, Capability::CodeGeneration);
        assert_eq!(cs.support_level, SupportLevel::Full);
        assert!((cs.confidence - 0.95).abs() < f64::EPSILON);
    }

    #[test]
    fn capability_support_none_level() {
        let cs = CapabilitySupport {
            capability: Capability::Vision,
            support_level: SupportLevel::None,
            confidence: 0.0,
        };
        assert_eq!(cs.support_level, SupportLevel::None);
    }

    // -----------------------------------------------------------------------
    // OperationalCharacteristics
    // -----------------------------------------------------------------------

    #[test]
    fn operational_characteristics_defaults() {
        let op = OperationalCharacteristics::default();
        assert!(op.latency.is_none());
        assert!(op.cost.is_none());
        assert_eq!(op.context_window, 0);
        assert!(!op.streaming);
        assert_eq!(op.locality, Locality::Unknown);
        assert_eq!(op.privacy_level, PrivacyLevel::None);
    }

    #[test]
    fn operational_characteristics_region() {
        let op = OperationalCharacteristics {
            region: Some("us-east-1".into()),
            ..Default::default()
        };
        assert_eq!(op.region.as_deref(), Some("us-east-1"));
    }

    #[test]
    fn operational_characteristics_full() {
        let op = OperationalCharacteristics {
            latency: Some(LatencyProfile {
                p50_ms: 350.0,
                p95_ms: 1200.0,
            }),
            cost: Some(CostProfile {
                per_input_token: 0.000_0025,
                per_output_token: 0.000_01,
                currency: "USD".into(),
            }),
            context_window: 128_000,
            streaming: true,
            locality: Locality::Cloud,
            privacy_level: PrivacyLevel::Medium,
            region: Some("us-east-1".into()),
        };
        assert!(op.latency.is_some());
        assert!((op.latency.as_ref().unwrap().p50_ms - 350.0).abs() < f64::EPSILON);
        assert!(op.cost.is_some());
        assert_eq!(op.context_window, 128_000);
        assert!(op.streaming);
        assert_eq!(op.locality, Locality::Cloud);
        assert_eq!(op.region.as_deref(), Some("us-east-1"));
    }

    #[test]
    fn latency_profile_defaults() {
        let lp = LatencyProfile::default();
        assert!((lp.p50_ms - 0.0).abs() < f64::EPSILON);
        assert!((lp.p95_ms - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn cost_profile_defaults() {
        let cp = CostProfile::default();
        assert!((cp.per_input_token - 0.0).abs() < f64::EPSILON);
        assert!((cp.per_output_token - 0.0).abs() < f64::EPSILON);
        assert_eq!(cp.currency, "USD");
    }

    // -----------------------------------------------------------------------
    // ModelLimits
    // -----------------------------------------------------------------------

    #[test]
    fn model_limits_defaults() {
        let ml = ModelLimits::default();
        assert_eq!(ml.max_context_tokens, 0);
        assert_eq!(ml.max_output_tokens, 0);
    }

    #[test]
    fn model_limits_construction() {
        let ml = ModelLimits {
            max_context_tokens: 128_000,
            max_output_tokens: 16_384,
        };
        assert_eq!(ml.max_context_tokens, 128_000);
        assert_eq!(ml.max_output_tokens, 16_384);
    }

    // -----------------------------------------------------------------------
    // ModelMetadata
    // -----------------------------------------------------------------------

    #[test]
    fn model_metadata_defaults() {
        let mm = ModelMetadata::default();
        assert!(mm.description.is_none());
        assert!(mm.tags.is_empty());
        assert!(mm.documentation_url.is_none());
        assert!(mm.release_date.is_none());
    }

    #[test]
    fn model_metadata_construction() {
        let mm = ModelMetadata {
            description: Some("A fast and capable model".into()),
            tags: vec!["fast".into(), "general".into()],
            documentation_url: Some("https://docs.example.com/model".into()),
            release_date: Some("2024-05-13".into()),
            confidence: Some(0.95),
        };
        assert_eq!(mm.description.as_deref(), Some("A fast and capable model"));
        assert_eq!(mm.tags.len(), 2);
        assert!(mm.documentation_url.is_some());
        assert!(mm.release_date.is_some());
        assert!((mm.confidence.unwrap() - 0.95).abs() < f64::EPSILON);
    }

    #[test]
    fn model_metadata_confidence_optional() {
        let mm = ModelMetadata::default();
        assert!(mm.confidence.is_none());
    }

    // -----------------------------------------------------------------------
    // ModelProfile
    // -----------------------------------------------------------------------

    #[test]
    fn model_profile_defaults() {
        let mp = ModelProfile::default();
        assert_eq!(mp.identity.id, "");
        assert!(mp.capabilities.is_empty());
        assert_eq!(mp.operational_characteristics.context_window, 0);
        assert_eq!(mp.limits.max_context_tokens, 0);
        assert!(mp.metadata.description.is_none());
    }

    #[test]
    fn model_profile_construction() {
        let mp = ModelProfile {
            identity: ModelIdentity {
                id: "claude-3-opus".into(),
                name: "Claude 3 Opus".into(),
                provider: "anthropic".into(),
                version: None,
                family: Some("claude-3".into()),
            },
            capabilities: vec![
                CapabilitySupport {
                    capability: Capability::CodeGeneration,
                    support_level: SupportLevel::Full,
                    confidence: 0.98,
                },
                CapabilitySupport {
                    capability: Capability::LongContext,
                    support_level: SupportLevel::Native,
                    confidence: 1.0,
                },
            ],
            operational_characteristics: OperationalCharacteristics {
                latency: None,
                cost: Some(CostProfile {
                    per_input_token: 0.000_015,
                    per_output_token: 0.000_075,
                    currency: "USD".into(),
                }),
                context_window: 200_000,
                streaming: true,
                locality: Locality::Cloud,
                privacy_level: PrivacyLevel::High,
                region: None,
            },
            limits: ModelLimits {
                max_context_tokens: 200_000,
                max_output_tokens: 4_096,
            },
            metadata: ModelMetadata {
                description: Some("Most capable Claude model".into()),
                tags: vec!["anthropic".into(), "flagship".into()],
                documentation_url: Some("https://docs.anthropic.com/claude".into()),
                release_date: Some("2024-03-04".into()),
                confidence: None,
            },
        };
        assert_eq!(mp.identity.id, "claude-3-opus");
        assert_eq!(mp.capabilities.len(), 2);
        assert!(mp.operational_characteristics.streaming);
        assert_eq!(mp.limits.max_context_tokens, 200_000);
    }

    #[test]
    fn model_profile_supports_capability() {
        let mp = ModelProfile {
            capabilities: vec![CapabilitySupport {
                capability: Capability::CodeGeneration,
                support_level: SupportLevel::Full,
                confidence: 1.0,
            }],
            ..Default::default()
        };
        assert!(mp.supports_capability(&Capability::CodeGeneration));
        assert!(!mp.supports_capability(&Capability::Vision));
    }

    #[test]
    fn model_profile_none_support_level_not_supported() {
        let mp = ModelProfile {
            capabilities: vec![CapabilitySupport {
                capability: Capability::Vision,
                support_level: SupportLevel::None,
                confidence: 0.0,
            }],
            ..Default::default()
        };
        assert!(!mp.supports_capability(&Capability::Vision));
    }

    #[test]
    fn model_profile_supports_by_name() {
        let mp = ModelProfile {
            capabilities: vec![CapabilitySupport {
                capability: Capability::CodeGeneration,
                support_level: SupportLevel::Full,
                confidence: 1.0,
            }],
            ..Default::default()
        };
        assert!(mp.supports_capability_by_name("code generation"));
        assert!(!mp.supports_capability_by_name("vision"));
    }

    #[test]
    fn model_profile_partial_support() {
        let mp = ModelProfile {
            capabilities: vec![CapabilitySupport {
                capability: Capability::Vision,
                support_level: SupportLevel::Partial,
                confidence: 0.5,
            }],
            ..Default::default()
        };
        assert!(mp.supports_capability(&Capability::Vision));
    }

    #[test]
    fn model_profile_multiple_capabilities() {
        let mp = ModelProfile {
            capabilities: vec![
                CapabilitySupport {
                    capability: Capability::CodeGeneration,
                    support_level: SupportLevel::Full,
                    confidence: 1.0,
                },
                CapabilitySupport {
                    capability: Capability::Translation,
                    support_level: SupportLevel::Full,
                    confidence: 0.9,
                },
                CapabilitySupport {
                    capability: Capability::Summarization,
                    support_level: SupportLevel::Native,
                    confidence: 1.0,
                },
            ],
            ..Default::default()
        };
        assert!(mp.supports_capability(&Capability::CodeGeneration));
        assert!(mp.supports_capability(&Capability::Translation));
        assert!(mp.supports_capability(&Capability::Summarization));
        assert!(!mp.supports_capability(&Capability::Vision));
    }
}
