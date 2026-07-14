use crate::pipeline::derived::DerivedProfile;
use crate::pipeline::intrinsic::IntrinsicProfile;

/// The canonical representation of a prompt after analysis.
///
/// Combines observable facts (IntrinsicProfile) with inferred
/// conclusions (DerivedProfile). Contains no analysis logic.
#[derive(Debug, Clone, PartialEq)]
pub struct PromptProfile {
    /// Observable properties extracted directly from the text.
    pub intrinsic: IntrinsicProfile,
    /// Inferred properties derived from intrinsic observations.
    pub derived: DerivedProfile,
}

#[cfg(test)]
mod tests {
    use crate::pipeline::intrinsic::IntrinsicProfile;
    use crate::pipeline::derived::DerivedProfile;

    use super::PromptProfile;

    #[test]
    fn prompt_profile_combines_intrinsic_and_derived() {
        let intrinsic = IntrinsicProfile {
            text: "hello".into(),
            word_count: 1,
            languages: vec![],
            frameworks: vec![],
            output_format: None,
            keywords: vec![],
            modality: "text".into(),
        };
        let derived = DerivedProfile {
            task_category: "general".into(),
            complexity: "low".into(),
            reasoning_depth: "shallow".into(),
            ambiguity: "low".into(),
        };
        let profile = PromptProfile {
            intrinsic: intrinsic.clone(),
            derived: derived.clone(),
        };
        assert_eq!(profile.intrinsic, intrinsic);
        assert_eq!(profile.derived, derived);
    }
}
