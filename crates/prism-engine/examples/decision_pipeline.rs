use prism_core::types::{
    Capability, CapabilitySupport, ModelIdentity, ModelProfile, Policy, Prompt, SupportLevel,
};
use prism_engine::engine::DecisionEngine;
use prism_registry::{DefaultModelRegistry, ModelRegistry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Initializing Prism Engine Playground ---\n");

    // 1. Create Engine
    let engine = DecisionEngine::default();

    // 2. Setup Registry & Populate Models
    let mut registry = DefaultModelRegistry::new();

    let model_a = ModelProfile {
        identity: ModelIdentity {
            id: "a".into(),
            name: "FastCode".into(),
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
            id: "b".into(),
            name: "DeepThink".into(),
            provider: "remote".into(),
            ..Default::default()
        },
        capabilities: vec![CapabilitySupport {
            capability: Capability::LogicalReasoning,
            support_level: SupportLevel::Full,
            confidence: 1.0,
        }],
        ..Default::default()
    };

    registry.register(model_a)?;
    registry.register(model_b)?;

    println!("Registry initialized with {} models.\n", registry.len());

    // 3. Define Prompt and Policy
    let prompt = Prompt {
        text: "Write a sorting algorithm in Python and explain the time complexity.".into(),
    };
    let policy = Policy::default(); // Uses default policy

    println!("Prompt: '{}'\n", prompt.text);

    // 4. Run Engine
    println!("Running pipeline...");
    let report = engine.evaluate(&prompt, &registry.list(), &policy)?;

    // 5. Output Result
    println!("\n--- Pipeline Result ---");
    println!(
        "Recommended Model: {} ({})",
        report.recommendation.model.identity.name, report.recommendation.model.identity.id
    );
    println!("Score: {:.2}", report.recommendation.score);
    println!("\nExplanation:");
    println!("{}", report.explanation.summary);

    Ok(())
}
