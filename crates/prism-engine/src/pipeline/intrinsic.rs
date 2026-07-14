use prism_core::error::PrismError;
use prism_core::types::Prompt;

/// Observable facts extracted directly from a prompt.
///
/// These are deterministic properties that can be identified without
/// inference or external knowledge.
#[derive(Debug, Clone, PartialEq)]
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
    /// The communication modality (e.g. "text", "code", "multimodal").
    pub modality: String,
}

/// Responsible for extracting observable properties from a normalized prompt.
pub trait IntrinsicExtractor {
    /// Extracts an IntrinsicProfile from the given prompt.
    fn extract(&self, prompt: &Prompt) -> Result<IntrinsicProfile, PrismError>;
}

/// A default extractor that uses simple keyword and pattern matching.
pub struct DefaultIntrinsicExtractor;

impl IntrinsicExtractor for DefaultIntrinsicExtractor {
    fn extract(&self, prompt: &Prompt) -> Result<IntrinsicProfile, PrismError> {
        let text = prompt.text.clone();
        let lower = text.to_lowercase();
        let word_count = text.split_whitespace().count();

        let mut languages = Vec::new();
        if lower.contains("python") || lower.contains("numpy") || lower.contains("pandas") {
            languages.push("python".to_string());
        }
        if lower.contains("javascript") || lower.contains("typescript") || lower.contains("node") {
            languages.push("javascript".to_string());
        }
        if lower.contains("rust") || lower.contains("cargo") {
            languages.push("rust".to_string());
        }
        if lower.contains("go ") || lower.contains("golang") {
            languages.push("go".to_string());
        }

        let mut frameworks = Vec::new();
        if lower.contains("react") || lower.contains("vue") || lower.contains("angular") {
            frameworks.push("frontend".to_string());
        }
        if lower.contains("django") || lower.contains("flask") || lower.contains("rails") {
            frameworks.push("backend".to_string());
        }

        let output_format = if lower.contains("json") {
            Some("json".to_string())
        } else if lower.contains("xml") {
            Some("xml".to_string())
        } else if lower.contains("markdown") || lower.contains("md") {
            Some("markdown".to_string())
        } else if lower.contains("csv") {
            Some("csv".to_string())
        } else {
            None
        };

        let mut keywords: Vec<String> = text
            .split_whitespace()
            .filter(|w| w.len() > 5)
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .filter(|w| !w.is_empty())
            .collect();
        keywords.sort();
        keywords.dedup();
        keywords.truncate(10);

        let modality = if languages.is_empty() && !text.contains("```") {
            "text".to_string()
        } else {
            "code".to_string()
        };

        Ok(IntrinsicProfile {
            text,
            word_count,
            languages,
            frameworks,
            output_format,
            keywords,
            modality,
        })
    }
}
