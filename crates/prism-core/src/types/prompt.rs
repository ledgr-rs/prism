/// Represents the user's original request before any analysis has been performed.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Prompt {
    /// The raw text of the request.
    pub text: String,
}
