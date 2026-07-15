pub use prism_core::types::PromptProfile;

#[cfg(test)]
mod tests {
    use crate::pipeline::derived::DerivedProfile;
    use crate::pipeline::intrinsic::IntrinsicProfile;

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
