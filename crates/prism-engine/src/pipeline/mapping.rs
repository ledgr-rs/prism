use prism_core::error::PrismError;
use prism_core::types::Capability;

use crate::pipeline::requirement::TaskRequirements;

/// Responsible for translating TaskRequirements into canonical Capability values.
///
/// This stage maps implementation-neutral requirements to Prism's
/// capability taxonomy. It contains no provider or model knowledge.
pub trait CapabilityMapper {
    /// Maps the inferred requirements to a list of canonical capabilities.
    fn map(&self, requirements: &TaskRequirements) -> Result<Vec<Capability>, PrismError>;
}

/// A default mapper that uses simple deterministic rules.
pub struct DefaultCapabilityMapper;

impl CapabilityMapper for DefaultCapabilityMapper {
    fn map(&self, requirements: &TaskRequirements) -> Result<Vec<Capability>, PrismError> {
        let mut capabilities = Vec::new();

        if requirements.needs_code_generation {
            capabilities.push(Capability::CodeGeneration);
        }
        if requirements.needs_reasoning {
            capabilities.push(Capability::LogicalReasoning);
        }
        if requirements.needs_planning {
            capabilities.push(Capability::MultiStepPlanning);
        }
        if requirements.needs_translation {
            capabilities.push(Capability::Translation);
        }
        if requirements.needs_writing {
            capabilities.push(Capability::Writing);
        }
        if requirements.needs_research {
            capabilities.push(Capability::GeneralKnowledge);
        }
        if requirements.needs_conversation {
            capabilities.push(Capability::Conversation);
        }
        if requirements.needs_tool_use {
            capabilities.push(Capability::ToolUse);
        }
        if requirements.needs_structured_output {
            capabilities.push(Capability::StructuredOutput);
        }

        for format in &requirements.output_formats {
            let cap = match format.as_str() {
                "json" => Capability::Json,
                "xml" => Capability::Xml,
                "yaml" | "yml" => Capability::Yaml,
                "markdown" | "md" => Capability::Markdown,
                "csv" | "table" => Capability::Tables,
                _ => continue,
            };
            if !capabilities.contains(&cap) {
                capabilities.push(cap);
            }
        }

        if capabilities.is_empty() {
            capabilities.push(Capability::General);
        }

        Ok(capabilities)
    }
}
