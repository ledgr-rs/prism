use prism_core::types::ModelProfile;

use crate::error::RegistryError;
use crate::registry::ModelRegistry;

pub struct DefaultModelRegistry {
    models: Vec<ModelProfile>,
}

impl DefaultModelRegistry {
    pub fn new() -> Self {
        Self { models: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { models: Vec::with_capacity(capacity) }
    }

    pub fn models(&self) -> &[ModelProfile] {
        &self.models
    }
}

impl Default for DefaultModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelRegistry for DefaultModelRegistry {
    fn register(&mut self, profile: ModelProfile) -> Result<(), RegistryError> {
        self.validate(&profile)?;

        if self.models.iter().any(|m| m.identity.id == profile.identity.id) {
            return Err(RegistryError::DuplicateModelId(profile.identity.id.clone()));
        }

        self.models.push(profile);
        Ok(())
    }

    fn list(&self) -> Vec<ModelProfile> {
        self.models.clone()
    }

    fn fetch(&self, id: &str) -> Option<ModelProfile> {
        self.models.iter().find(|m| m.identity.id == id).cloned()
    }

    fn validate(&self, profile: &ModelProfile) -> Result<(), RegistryError> {
        if profile.identity.id.trim().is_empty() {
            return Err(RegistryError::InvalidModel(
                "Model id must not be empty".to_string(),
            ));
        }
        if profile.capabilities.is_empty() {
            return Err(RegistryError::InvalidModel(
                "Model must have at least one capability".to_string(),
            ));
        }
        Ok(())
    }

    fn len(&self) -> usize {
        self.models.len()
    }

    fn is_empty(&self) -> bool {
        self.models.is_empty()
    }
}
