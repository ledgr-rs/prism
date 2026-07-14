/// The score assigned to a candidate model during the scoring stage.
///
/// Contains the final score along with supporting evidence, bonuses,
/// and penalties so that every scoring decision is explainable and
/// reproducible.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CandidateScore {
    /// The final computed score for this candidate.
    pub final_score: f64,
    /// Confidence in the score (0.0 – 1.0).
    pub confidence: f64,
    /// Individual pieces of evidence that contributed to the score.
    pub evidence: Vec<String>,
    /// Bonuses applied during scoring.
    pub bonuses: Vec<String>,
    /// Penalties applied during scoring.
    pub penalties: Vec<String>,
}
