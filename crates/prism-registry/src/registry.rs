use prism_core::types::ModelProfile;

use crate::error::RegistryError;

pub trait ModelRegistry {
    fn register(&mut self, profile: ModelProfile) -> Result<(), RegistryError>;
    fn list(&self) -> Vec<ModelProfile>;
    fn fetch(&self, id: &str) -> Option<ModelProfile>;
    fn validate(&self, profile: &ModelProfile) -> Result<(), RegistryError>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
