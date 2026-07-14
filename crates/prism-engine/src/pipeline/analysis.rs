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
