/// Represents the user's original request before any analysis has been performed.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Prompt {
    /// The raw text of the request.
    pub text: String,
}

/// Observable facts extracted directly from a prompt.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IntrinsicProfile {
    /// The original raw text.
    pub text: String,
    /// Approximate word count.
    pub word_count: usize,
    /// Programming languages detected in the prompt.
    pub languages: Vec<String>,
    /// Frameworks or libraries mentioned.
    pub frameworks: Vec<String>,
    /// Expected output format, if detectable.
    pub output_format: Option<String>,
    /// Notable keywords found in the prompt.
    pub keywords: Vec<String>,
    /// The communication modality.
    pub modality: String,
}

/// Inferred properties derived from observable prompt facts.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DerivedProfile {
    /// The inferred category of the task.
    pub task_category: String,
    /// Estimated complexity level.
    pub complexity: String,
    /// Required depth of reasoning.
    pub reasoning_depth: String,
    /// Estimated ambiguity of the request.
    pub ambiguity: String,
}

/// The complete prompt analysis artifact produced by the engine.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PromptProfile {
    /// Observable prompt facts.
    pub intrinsic: IntrinsicProfile,
    /// Inferred conclusions from those facts.
    pub derived: DerivedProfile,
}
