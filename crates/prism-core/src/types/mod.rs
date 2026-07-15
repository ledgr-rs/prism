pub mod candidate_score;
pub use candidate_score::CandidateScore;

pub mod capability;
pub use capability::{Capability, CapabilityProfile, CapabilityRequirement, Priority};

pub mod explanation;
pub use explanation::{
    CapabilityMatch, Evidence, Explanation, MatchStatus, PolicyDecision, RejectedAlternative,
};

pub mod model;
pub use model::{
    CapabilitySupport, CostProfile, LatencyProfile, Locality, ModelIdentity, ModelLimits,
    ModelMetadata, ModelProfile, OperationalCharacteristics, PrivacyLevel, SupportLevel,
};

pub mod policy;
pub use policy::{Policy, PolicyRule};

pub mod prompt;
pub use prompt::{DerivedProfile, IntrinsicProfile, Prompt, PromptProfile};

pub mod recommendation;
pub use recommendation::Recommendation;

pub mod report;
pub use report::DecisionReport;
