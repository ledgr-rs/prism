use prism_core::types::{
    Capability, CapabilitySupport, ModelIdentity, ModelProfile, Policy, Prompt, SupportLevel,
};
use prism_engine::engine::DecisionEngine;
use prism_registry::{DefaultModelRegistry, ModelRegistry};

fn setup_engine_and_registry() -> (DecisionEngine, DefaultModelRegistry) {
    let engine = DecisionEngine::default();
    let mut registry = DefaultModelRegistry::new();

    // Add some diverse models
    let model_a = ModelProfile {
        identity: ModelIdentity {
            id: "rust-pro".into(),
            name: "RustPro".into(),
            provider: "local".into(),
            ..Default::default()
        },
        capabilities: vec![CapabilitySupport {
            capability: Capability::CodeGeneration,
            support_level: SupportLevel::Full,
            confidence: 1.0,
        }],
        ..Default::default()
    };

    let model_b = ModelProfile {
        identity: ModelIdentity {
            id: "general-bot".into(),
            name: "GeneralBot".into(),
            provider: "remote".into(),
            ..Default::default()
        },
        capabilities: vec![CapabilitySupport {
            capability: Capability::General,
            support_level: SupportLevel::Full,
            confidence: 1.0,
        }],
        ..Default::default()
    };

    registry.register(model_a).unwrap();
    registry.register(model_b).unwrap();

    (engine, registry)
}

#[test]
fn test_rust_api_generation_golden() {
    let (engine, registry) = setup_engine_and_registry();
    let prompt = Prompt {
        text: "Write a Rust function for Fibonacci.".into(),
    };
    let policy = Policy::default();

    let report = engine.evaluate(&prompt, &registry.list(), &policy).unwrap();

    assert_eq!(report.recommendation.model.identity.id, "rust-pro");
    assert!(!report.explanation.summary.is_empty());
}
