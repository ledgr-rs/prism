/// Represents the capabilities and properties of a specific model.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModelProfile {
    /// The unique identifier of the model.
    pub id: String,
    /// The capabilities this model possesses.
    pub capabilities: Vec<String>,
}
