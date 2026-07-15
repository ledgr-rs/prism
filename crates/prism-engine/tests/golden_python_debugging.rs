use prism_core::types::{
    Capability, CapabilitySupport, ModelIdentity, ModelProfile, Policy, Prompt, SupportLevel,
};
use prism_engine::engine::DecisionEngine;
use prism_registry::{DefaultModelRegistry, ModelRegistry};

fn setup_engine_and_registry() -> (DecisionEngine, DefaultModelRegistry) {
    let engine = DecisionEngine::default();
    let mut registry = DefaultModelRegistry::new();

    let model_a = ModelProfile {
        identity: ModelIdentity {
            id: "python-debug".into(),
            name: "PyDebug".into(),
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
fn test_python_debugging_golden() {
    let (engine, registry) = setup_engine_and_registry();
    let prompt = Prompt {
        text: "Debug this Python loop index error.".into(),
    };
    let policy = Policy::default();

    let report = engine.evaluate(&prompt, &registry.list(), &policy).unwrap();

    assert_eq!(report.recommendation.model.identity.id, "python-debug");
    assert!(!report.explanation.summary.is_empty());
}
