use prism_core::types::{
    Capability, CapabilitySupport, CostProfile, LatencyProfile, Locality, ModelIdentity,
    ModelLimits, ModelMetadata, ModelProfile, OperationalCharacteristics, PrivacyLevel,
    SupportLevel,
};
use prism_registry::{DefaultModelRegistry, ModelRegistry};

pub fn load_models() -> Vec<ModelProfile> {
    let mut registry = DefaultModelRegistry::new();

    for model in [
        model(
            "prism-fast-code",
            "Prism Fast Code",
            "local",
            &[
                Capability::CodeGeneration,
                Capability::Debugging,
                Capability::Testing,
            ],
            0.95,
            24_000,
            90.0,
            PrivacyLevel::High,
            Locality::Local,
        ),
        model(
            "prism-deep-reason",
            "Prism Deep Reason",
            "remote",
            &[
                Capability::LogicalReasoning,
                Capability::MathematicalReasoning,
                Capability::MultiStepPlanning,
                Capability::Explanation,
            ],
            0.98,
            128_000,
            420.0,
            PrivacyLevel::Medium,
            Locality::Cloud,
        ),
        model(
            "prism-generalist",
            "Prism Generalist",
            "remote",
            &[
                Capability::GeneralKnowledge,
                Capability::Writing,
                Capability::Summarization,
                Capability::Translation,
                Capability::StructuredOutput,
            ],
            0.92,
            64_000,
            180.0,
            PrivacyLevel::Low,
            Locality::Cloud,
        ),
        model(
            "prism-structured",
            "Prism Structured",
            "hybrid",
            &[
                Capability::StructuredOutput,
                Capability::Json,
                Capability::Tables,
                Capability::FunctionCalling,
                Capability::ToolUse,
            ],
            0.94,
            96_000,
            210.0,
            PrivacyLevel::Medium,
            Locality::Hybrid,
        ),
        model(
            "prism-safe-local",
            "Prism Safe Local",
            "local",
            &[
                Capability::Privacy,
                Capability::Compliance,
                Capability::SafeCompletion,
                Capability::General,
            ],
            0.9,
            32_000,
            120.0,
            PrivacyLevel::High,
            Locality::Local,
        ),
    ] {
        let _ = registry.register(model);
    }

    registry.list()
}

fn model(
    id: &str,
    name: &str,
    provider: &str,
    capabilities: &[Capability],
    confidence: f64,
    context_window: usize,
    p95_ms: f64,
    privacy_level: PrivacyLevel,
    locality: Locality,
) -> ModelProfile {
    ModelProfile {
        identity: ModelIdentity {
            id: id.into(),
            name: name.into(),
            provider: provider.into(),
            version: Some("0.1".into()),
            family: Some("prism-reference".into()),
        },
        capabilities: capabilities
            .iter()
            .cloned()
            .map(|capability| CapabilitySupport {
                capability,
                support_level: SupportLevel::Full,
                confidence,
            })
            .collect(),
        operational_characteristics: OperationalCharacteristics {
            latency: Some(LatencyProfile {
                p50_ms: p95_ms / 2.0,
                p95_ms,
            }),
            cost: Some(CostProfile {
                per_input_token: 0.000001,
                per_output_token: 0.000004,
                currency: "USD".into(),
            }),
            context_window,
            streaming: true,
            locality,
            privacy_level,
            region: Some("portable".into()),
        },
        limits: ModelLimits {
            max_context_tokens: context_window,
            max_output_tokens: 8192,
        },
        metadata: ModelMetadata {
            description: Some("Bundled reference profile for the Prism TUI.".into()),
            tags: vec!["reference".into(), "portable".into()],
            documentation_url: None,
            release_date: Some("2026-07-15".into()),
            confidence: Some(confidence),
        },
    }
}
