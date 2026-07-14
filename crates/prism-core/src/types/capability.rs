use std::fmt;

/// The canonical capability taxonomy for Prism.
///
/// Every capability referenced throughout the engine originates from this enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Capability {
    /// A default capability representing a general task. Used as a fallback.
    General,
    // Reasoning
    LogicalReasoning,
    MathematicalReasoning,
    MultiStepPlanning,
    DecisionMaking,
    // Coding
    CodeGeneration,
    CodeReview,
    Debugging,
    Refactoring,
    Architecture,
    Testing,
    // Knowledge
    GeneralKnowledge,
    ScientificKnowledge,
    MedicalKnowledge,
    LegalKnowledge,
    FinancialKnowledge,
    // Communication
    Writing,
    Summarization,
    Translation,
    Explanation,
    Conversation,
    // Context
    LongContext,
    ConversationMemory,
    MultiDocumentReasoning,
    // Modalities
    Vision,
    Audio,
    Video,
    Multimodal,
    // Tooling
    FunctionCalling,
    ToolUse,
    AgenticPlanning,
    Retrieval,
    // Output
    StructuredOutput,
    Json,
    Markdown,
    Tables,
    Xml,
    Yaml,
    // Reliability
    DeterministicOutput,
    LowLatency,
    HighThroughput,
    CostEfficiency,
    // Safety
    Privacy,
    Compliance,
    SafeCompletion,
}

impl Default for Capability {
    fn default() -> Self {
        Capability::General
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Capability::General => "general",
            // Reasoning
            Capability::LogicalReasoning => "logical reasoning",
            Capability::MathematicalReasoning => "mathematical reasoning",
            Capability::MultiStepPlanning => "multi-step planning",
            Capability::DecisionMaking => "decision making",
            // Coding
            Capability::CodeGeneration => "code generation",
            Capability::CodeReview => "code review",
            Capability::Debugging => "debugging",
            Capability::Refactoring => "refactoring",
            Capability::Architecture => "architecture",
            Capability::Testing => "testing",
            // Knowledge
            Capability::GeneralKnowledge => "general knowledge",
            Capability::ScientificKnowledge => "scientific knowledge",
            Capability::MedicalKnowledge => "medical knowledge",
            Capability::LegalKnowledge => "legal knowledge",
            Capability::FinancialKnowledge => "financial knowledge",
            // Communication
            Capability::Writing => "writing",
            Capability::Summarization => "summarization",
            Capability::Translation => "translation",
            Capability::Explanation => "explanation",
            Capability::Conversation => "conversation",
            // Context
            Capability::LongContext => "long context",
            Capability::ConversationMemory => "conversation memory",
            Capability::MultiDocumentReasoning => "multi-document reasoning",
            // Modalities
            Capability::Vision => "vision",
            Capability::Audio => "audio",
            Capability::Video => "video",
            Capability::Multimodal => "multimodal",
            // Tooling
            Capability::FunctionCalling => "function calling",
            Capability::ToolUse => "tool use",
            Capability::AgenticPlanning => "agentic planning",
            Capability::Retrieval => "retrieval",
            // Output
            Capability::StructuredOutput => "structured output",
            Capability::Json => "json",
            Capability::Markdown => "markdown",
            Capability::Tables => "tables",
            Capability::Xml => "xml",
            Capability::Yaml => "yaml",
            // Reliability
            Capability::DeterministicOutput => "deterministic output",
            Capability::LowLatency => "low latency",
            Capability::HighThroughput => "high throughput",
            Capability::CostEfficiency => "cost efficiency",
            // Safety
            Capability::Privacy => "privacy",
            Capability::Compliance => "compliance",
            Capability::SafeCompletion => "safe completion",
        };
        write!(f, "{}", name)
    }
}

/// Represents how essential a capability is for the task.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Priority {
    /// The capability is essential — the task cannot be completed without it.
    #[default]
    Required,
    /// The capability significantly improves the result.
    Preferred,
    /// The capability may be beneficial but is not critical.
    Optional,
}

/// A single capability requirement with supporting metadata.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CapabilityRequirement {
    /// The canonical capability from the taxonomy.
    pub capability: Capability,
    /// How essential this capability is.
    pub priority: Priority,
    /// Relative importance weight (0.0 – 1.0).
    pub weight: f64,
    /// Confidence in the extraction (0.0 – 1.0).
    pub confidence: f64,
    /// Human-readable explanation for why this capability was inferred.
    pub reason: String,
}

/// Represents the capabilities required to complete a task.
///
/// The profile is the output of Capability Extraction and the primary input
/// for model scoring and selection.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CapabilityProfile {
    /// The complete set of capability requirements.
    pub requirements: Vec<CapabilityRequirement>,
}
