use prism_core::error::PrismError;
use prism_core::types::{Capability, CapabilityRequirement, Priority};

/// Responsible for assigning priority metadata to capabilities.
///
/// This stage takes a flat list of required capabilities and produces
/// structured CapabilityRequirements with priority, weight, confidence,
/// and a human-readable reason.
pub trait CapabilityPrioritizer {
    /// Assigns priority metadata to each capability.
    fn prioritize(
        &self,
        capabilities: Vec<Capability>,
    ) -> Result<Vec<CapabilityRequirement>, PrismError>;
}

/// A default prioritizer that uses simple deterministic rules.
pub struct DefaultCapabilityPrioritizer;

impl CapabilityPrioritizer for DefaultCapabilityPrioritizer {
    fn prioritize(
        &self,
        capabilities: Vec<Capability>,
    ) -> Result<Vec<CapabilityRequirement>, PrismError> {
        let requirements: Vec<CapabilityRequirement> = capabilities
            .into_iter()
            .map(|cap| {
                let (priority, weight, confidence) = match &cap {
                    // Coding capabilities are typically high priority
                    Capability::CodeGeneration
                    | Capability::Debugging
                    | Capability::Architecture => {
                        (Priority::Required, 0.95, 0.90)
                    }
                    Capability::CodeReview
                    | Capability::Refactoring
                    | Capability::Testing => {
                        (Priority::Preferred, 0.70, 0.75)
                    }

                    // Reasoning capabilities
                    Capability::LogicalReasoning
                    | Capability::MathematicalReasoning => {
                        (Priority::Required, 0.90, 0.85)
                    }
                    Capability::MultiStepPlanning
                    | Capability::DecisionMaking => {
                        (Priority::Preferred, 0.75, 0.70)
                    }

                    // Communication
                    Capability::Translation
                    | Capability::Writing
                    | Capability::Summarization => {
                        (Priority::Required, 0.85, 0.80)
                    }
                    Capability::Explanation
                    | Capability::Conversation => {
                        (Priority::Preferred, 0.65, 0.70)
                    }

                    // Knowledge
                    Capability::GeneralKnowledge
                    | Capability::ScientificKnowledge
                    | Capability::MedicalKnowledge
                    | Capability::LegalKnowledge
                    | Capability::FinancialKnowledge => {
                        (Priority::Preferred, 0.60, 0.60)
                    }

                    // Output formatting
                    Capability::StructuredOutput
                    | Capability::Json
                    | Capability::Xml
                    | Capability::Yaml
                    | Capability::Markdown
                    | Capability::Tables => {
                        if cap == Capability::StructuredOutput {
                            (Priority::Required, 0.85, 0.80)
                        } else {
                            (Priority::Optional, 0.40, 0.50)
                        }
                    }

                    // Tooling
                    Capability::ToolUse
                    | Capability::FunctionCalling
                    | Capability::AgenticPlanning
                    | Capability::Retrieval => {
                        (Priority::Preferred, 0.70, 0.65)
                    }

                    // Context
                    Capability::LongContext
                    | Capability::ConversationMemory
                    | Capability::MultiDocumentReasoning => {
                        (Priority::Optional, 0.50, 0.40)
                    }

                    // Modalities
                    Capability::Vision
                    | Capability::Audio
                    | Capability::Video
                    | Capability::Multimodal => {
                        (Priority::Optional, 0.50, 0.40)
                    }

                    // Reliability
                    Capability::DeterministicOutput
                    | Capability::LowLatency
                    | Capability::HighThroughput
                    | Capability::CostEfficiency => {
                        (Priority::Optional, 0.35, 0.30)
                    }

                    // Safety
                    Capability::Privacy
                    | Capability::Compliance
                    | Capability::SafeCompletion => {
                        (Priority::Optional, 0.30, 0.30)
                    }

                    // General fallback
                    Capability::General => {
                        (Priority::Optional, 0.20, 0.50)
                    }
                };

                let reason = format!("Capability '{}' inferred from prompt analysis", cap);

                CapabilityRequirement {
                    capability: cap,
                    priority,
                    weight,
                    confidence,
                    reason,
                }
            })
            .collect();

        Ok(requirements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prism_core::types::Capability;

    #[test]
    fn prioritizer_assigns_required_to_code_generation() {
        let prioritizer = DefaultCapabilityPrioritizer;
        let reqs = prioritizer.prioritize(vec![Capability::CodeGeneration]).unwrap();
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].priority, Priority::Required);
        assert!(reqs[0].weight > 0.9);
    }

    #[test]
    fn prioritizer_assigns_optional_to_general() {
        let prioritizer = DefaultCapabilityPrioritizer;
        let reqs = prioritizer.prioritize(vec![Capability::General]).unwrap();
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].priority, Priority::Optional);
        assert_eq!(reqs[0].weight, 0.20);
    }

    #[test]
    fn prioritizer_includes_reason_string() {
        let prioritizer = DefaultCapabilityPrioritizer;
        let reqs = prioritizer.prioritize(vec![Capability::Writing]).unwrap();
        assert!(reqs[0].reason.contains("writing"));
        assert!(reqs[0].reason.contains("inferred from prompt analysis"));
    }

    #[test]
    fn prioritizer_handles_multiple_capabilities() {
        let prioritizer = DefaultCapabilityPrioritizer;
        let caps = vec![
            Capability::CodeGeneration,
            Capability::Translation,
            Capability::General,
        ];
        let reqs = prioritizer.prioritize(caps).unwrap();
        assert_eq!(reqs.len(), 3);
        assert!(reqs.iter().any(|r| r.capability == Capability::CodeGeneration));
        assert!(reqs.iter().any(|r| r.capability == Capability::Translation));
        assert!(reqs.iter().any(|r| r.capability == Capability::General));
    }

    #[test]
    fn prioritizer_empty_input_yields_empty_output() {
        let prioritizer = DefaultCapabilityPrioritizer;
        let reqs = prioritizer.prioritize(vec![]).unwrap();
        assert!(reqs.is_empty());
    }
}
