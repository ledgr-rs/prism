use prism_core::error::PrismError;
use prism_core::types::Prompt;

pub use prism_core::types::IntrinsicProfile;

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
            .map(|w| {
                w.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase()
            })
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

#[cfg(test)]
mod tests {
    use super::*;
    use prism_core::types::Prompt;

    #[test]
    fn intrinsic_extracts_word_count() {
        let extractor = DefaultIntrinsicExtractor;
        let prompt = Prompt {
            text: "hello world from rust".into(),
        };
        let profile = extractor.extract(&prompt).unwrap();
        assert_eq!(profile.word_count, 4);
    }

    #[test]
    fn intrinsic_detects_programming_language() {
        let extractor = DefaultIntrinsicExtractor;
        let prompt = Prompt {
            text: "Write a Python function using numpy".into(),
        };
        let profile = extractor.extract(&prompt).unwrap();
        assert!(profile.languages.contains(&"python".to_string()));
    }

    #[test]
    fn intrinsic_detects_output_format() {
        let extractor = DefaultIntrinsicExtractor;
        let prompt = Prompt {
            text: "Return the result as JSON".into(),
        };
        let profile = extractor.extract(&prompt).unwrap();
        assert_eq!(profile.output_format, Some("json".to_string()));
    }

    #[test]
    fn intrinsic_sets_modality_to_text_for_plain_prompts() {
        let extractor = DefaultIntrinsicExtractor;
        let prompt = Prompt {
            text: "Tell me a story".into(),
        };
        let profile = extractor.extract(&prompt).unwrap();
        assert_eq!(profile.modality, "text");
    }

    #[test]
    fn intrinsic_sets_modality_to_code_when_language_detected() {
        let extractor = DefaultIntrinsicExtractor;
        let prompt = Prompt {
            text: "Write a Rust function".into(),
        };
        let profile = extractor.extract(&prompt).unwrap();
        assert_eq!(profile.modality, "code");
    }

    #[test]
    fn intrinsic_extracts_keywords() {
        let extractor = DefaultIntrinsicExtractor;
        let prompt = Prompt {
            text: "implement a sorting algorithm efficiently".into(),
        };
        let profile = extractor.extract(&prompt).unwrap();
        assert!(profile.keywords.contains(&"sorting".to_string()));
    }
}
