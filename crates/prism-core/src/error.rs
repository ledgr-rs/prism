use std::fmt;

/// Central error type for the Prism core domain.
#[derive(Debug, Clone, PartialEq)]
pub enum PrismError {
    /// Occurs when a required piece of information is missing.
    MissingInformation(String),
    /// Occurs when an operation is performed on an incompatible type.
    IncompatibleType(String),
    /// A general internal error that cannot be more specifically categorized.
    Internal(String),
}

impl fmt::Display for PrismError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrismError::MissingInformation(msg) => write!(f, "Missing information: {}", msg),
            PrismError::IncompatibleType(msg) => write!(f, "Incompatible type: {}", msg),
            PrismError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for PrismError {}
