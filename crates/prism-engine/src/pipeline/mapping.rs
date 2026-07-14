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

#[cfg(test)]
mod tests {
    use super::*;
    use prism_core::types::Capability;

    fn requirements(
        code: bool,
        reasoning: bool,
        structured: bool,
        writing: bool,
    ) -> TaskRequirements {
        TaskRequirements {
            needs_code_generation: code,
            needs_reasoning: reasoning,
            needs_structured_output: structured,
            needs_writing: writing,
            ..Default::default()
        }
    }

    #[test]
    fn mapper_maps_code_generation() {
        let reqs = requirements(true, false, false, false);
        let mapper = DefaultCapabilityMapper;
        let caps = mapper.map(&reqs).unwrap();
        assert!(caps.contains(&Capability::CodeGeneration));
    }

    #[test]
    fn mapper_maps_multiple_capabilities() {
        let reqs = requirements(true, true, true, false);
        let mapper = DefaultCapabilityMapper;
        let caps = mapper.map(&reqs).unwrap();
        assert!(caps.contains(&Capability::CodeGeneration));
        assert!(caps.contains(&Capability::LogicalReasoning));
        assert!(caps.contains(&Capability::StructuredOutput));
    }

    #[test]
    fn mapper_returns_general_when_no_requirements() {
        let reqs = requirements(false, false, false, false);
        let mapper = DefaultCapabilityMapper;
        let caps = mapper.map(&reqs).unwrap();
        assert_eq!(caps, vec![Capability::General]);
    }

    #[test]
    fn mapper_maps_translation() {
        let reqs = TaskRequirements {
            needs_translation: true,
            ..Default::default()
        };
        let mapper = DefaultCapabilityMapper;
        let caps = mapper.map(&reqs).unwrap();
        assert!(caps.contains(&Capability::Translation));
    }

    #[test]
    fn mapper_maps_output_formats() {
        let reqs = TaskRequirements {
            output_formats: vec!["json".into(), "xml".into()],
            ..Default::default()
        };
        let mapper = DefaultCapabilityMapper;
        let caps = mapper.map(&reqs).unwrap();
        assert!(caps.contains(&Capability::Json));
        assert!(caps.contains(&Capability::Xml));
    }
}
