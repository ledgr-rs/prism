pub mod default;
pub mod error;
pub mod registry;

pub use default::DefaultModelRegistry;
pub use error::RegistryError;
pub use registry::ModelRegistry;

#[cfg(test)]
mod tests {
    use prism_core::types::{Capability, CapabilitySupport, ModelIdentity, ModelProfile, SupportLevel};

    use super::*;

    fn sample_model() -> ModelProfile {
        ModelProfile {
            identity: ModelIdentity {
                id: "model-a".into(),
                name: "Model A".into(),
                provider: "test".into(),
                version: None,
                family: None,
            },
            capabilities: vec![
                CapabilitySupport {
                    capability: Capability::CodeGeneration,
                    support_level: SupportLevel::Full,
                    confidence: 1.0,
                },
                CapabilitySupport {
                    capability: Capability::LogicalReasoning,
                    support_level: SupportLevel::Full,
                    confidence: 1.0,
                },
            ],
            ..Default::default()
        }
    }

    fn model_with_id(id: &str) -> ModelProfile {
        ModelProfile {
            identity: ModelIdentity {
                id: id.into(),
                name: id.into(),
                provider: "test".into(),
                version: None,
                family: None,
            },
            capabilities: vec![CapabilitySupport {
                capability: Capability::General,
                support_level: SupportLevel::Full,
                confidence: 1.0,
            }],
            ..Default::default()
        }
    }

    #[test]
    fn registration_succeeds() {
        let mut registry = DefaultModelRegistry::new();
        let model = sample_model();
        let result = registry.register(model.clone());
        assert!(result.is_ok());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registration_rejects_duplicate() {
        let mut registry = DefaultModelRegistry::new();
        let model = sample_model();
        registry.register(model.clone()).unwrap();
        let result = registry.register(model);
        assert_eq!(result, Err(RegistryError::DuplicateModelId("model-a".into())));
    }

    #[test]
    fn fetch_returns_registered_model() {
        let mut registry = DefaultModelRegistry::new();
        let model = sample_model();
        registry.register(model.clone()).unwrap();
        let fetched = registry.fetch("model-a");
        assert_eq!(fetched, Some(model));
    }

    #[test]
    fn fetch_returns_none_for_missing() {
        let registry = DefaultModelRegistry::new();
        let fetched = registry.fetch("nonexistent");
        assert_eq!(fetched, None);
    }

    #[test]
    fn list_returns_all_models() {
        let mut registry = DefaultModelRegistry::new();
        let model_a = sample_model();
        let model_b = model_with_id("model-b");
        registry.register(model_a.clone()).unwrap();
        registry.register(model_b.clone()).unwrap();
        let models = registry.list();
        assert_eq!(models.len(), 2);
        assert!(models.contains(&model_a));
        assert!(models.contains(&model_b));
    }

    #[test]
    fn validation_rejects_empty_id() {
        let registry = DefaultModelRegistry::new();
        let model = ModelProfile {
            identity: ModelIdentity {
                id: "".into(),
                name: "".into(),
                provider: "test".into(),
                version: None,
                family: None,
            },
            capabilities: vec![CapabilitySupport {
                capability: Capability::LogicalReasoning,
                support_level: SupportLevel::Full,
                confidence: 1.0,
            }],
            ..Default::default()
        };
        let result = registry.validate(&model);
        assert_eq!(
            result,
            Err(RegistryError::InvalidModel(
                "Model id must not be empty".to_string()
            ))
        );
    }

    #[test]
    fn validation_rejects_whitespace_id() {
        let registry = DefaultModelRegistry::new();
        let model = ModelProfile {
            identity: ModelIdentity {
                id: "   ".into(),
                name: "".into(),
                provider: "test".into(),
                version: None,
                family: None,
            },
            capabilities: vec![CapabilitySupport {
                capability: Capability::LogicalReasoning,
                support_level: SupportLevel::Full,
                confidence: 1.0,
            }],
            ..Default::default()
        };
        let result = registry.validate(&model);
        assert_eq!(
            result,
            Err(RegistryError::InvalidModel(
                "Model id must not be empty".to_string()
            ))
        );
    }

    #[test]
    fn validation_rejects_empty_capabilities() {
        let registry = DefaultModelRegistry::new();
        let model = ModelProfile {
            identity: ModelIdentity {
                id: "model-a".into(),
                name: "Model A".into(),
                provider: "test".into(),
                version: None,
                family: None,
            },
            capabilities: vec![],
            ..Default::default()
        };
        let result = registry.validate(&model);
        assert_eq!(
            result,
            Err(RegistryError::InvalidModel(
                "Model must have at least one capability".to_string()
            ))
        );
    }

    #[test]
    fn registration_rejects_invalid_model() {
        let mut registry = DefaultModelRegistry::new();
        let model = ModelProfile {
            identity: ModelIdentity {
                id: "".into(),
                name: "".into(),
                provider: "test".into(),
                version: None,
                family: None,
            },
            capabilities: vec![CapabilitySupport {
                capability: Capability::LogicalReasoning,
                support_level: SupportLevel::Full,
                confidence: 1.0,
            }],
            ..Default::default()
        };
        let result = registry.register(model);
        assert!(result.is_err());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn empty_registry_is_empty() {
        let registry = DefaultModelRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn non_empty_registry_is_not_empty() {
        let mut registry = DefaultModelRegistry::new();
        registry.register(sample_model()).unwrap();
        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn with_capacity_creates_empty_registry() {
        let registry = DefaultModelRegistry::with_capacity(10);
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn models_returns_internal_slice() {
        let mut registry = DefaultModelRegistry::new();
        registry.register(sample_model()).unwrap();
        let models = registry.models();
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].identity.id, "model-a");
    }

    #[test]
    fn list_is_defensive_copy() {
        let mut registry = DefaultModelRegistry::new();
        registry.register(sample_model()).unwrap();
        let mut listed = registry.list();
        listed.push(model_with_id("injected"));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn immutability_during_evaluation() {
        let mut registry = DefaultModelRegistry::new();
        registry.register(sample_model()).unwrap();

        let models = registry.list();
        let _models_ref: &[ModelProfile] = &models;

        assert_eq!(registry.len(), 1);
        registry.register(model_with_id("model-b")).unwrap();
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn multiple_registrations_increment_count() {
        let mut registry = DefaultModelRegistry::new();
        for i in 0..5 {
            registry.register(model_with_id(&format!("model-{}", i))).unwrap();
        }
        assert_eq!(registry.len(), 5);
    }

    #[test]
    fn display_error_messages() {
        let err = RegistryError::DuplicateModelId("test".into());
        assert_eq!(format!("{}", err), "Duplicate model id: test");

        let err = RegistryError::InvalidModel("bad".into());
        assert_eq!(format!("{}", err), "Invalid model: bad");

        let err = RegistryError::ModelNotFound("missing".into());
        assert_eq!(format!("{}", err), "Model not found: missing");

        let err = RegistryError::Internal("oops".into());
        assert_eq!(format!("{}", err), "Internal error: oops");
    }
}

