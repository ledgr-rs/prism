use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RegistryError {
    DuplicateModelId(String),
    InvalidModel(String),
    ModelNotFound(String),
    Internal(String),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::DuplicateModelId(id) => write!(f, "Duplicate model id: {}", id),
            RegistryError::InvalidModel(msg) => write!(f, "Invalid model: {}", msg),
            RegistryError::ModelNotFound(id) => write!(f, "Model not found: {}", id),
            RegistryError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for RegistryError {}
