use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeclarationFreshness {
    Fresh,
    PotentiallyStale,
    Stale,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationFreshnessProbe {
    pub covered_commit_hash: Option<String>,
    pub current_commit_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationFreshnessReport {
    pub status: DeclarationFreshness,
    pub reasons: Vec<DeclarationFreshnessReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeclarationFreshnessReason {
    MissingCoveredCommit,
    CoveredCommitUnavailable,
    CoveredCommitDiffers,
}

impl DeclarationFreshnessProbe {
    #[must_use]
    pub fn evaluate(&self) -> DeclarationFreshnessReport {
        let mut reasons = Vec::new();
        match (&self.covered_commit_hash, &self.current_commit_hash) {
            (None, _) => reasons.push(DeclarationFreshnessReason::MissingCoveredCommit),
            (Some(_), None) => {
                reasons.push(DeclarationFreshnessReason::CoveredCommitUnavailable);
            }
            (Some(covered), Some(current)) if covered != current => {
                reasons.push(DeclarationFreshnessReason::CoveredCommitDiffers);
            }
            _ => {}
        }
        let status = if stale(&reasons) {
            DeclarationFreshness::Stale
        } else if reasons
            .contains(&DeclarationFreshnessReason::CoveredCommitUnavailable)
        {
            DeclarationFreshness::Unknown
        } else if reasons.contains(&DeclarationFreshnessReason::MissingCoveredCommit) {
            DeclarationFreshness::PotentiallyStale
        } else {
            DeclarationFreshness::Fresh
        };
        DeclarationFreshnessReport { status, reasons }
    }
}

fn stale(reasons: &[DeclarationFreshnessReason]) -> bool {
    reasons.iter().any(|reason| {
        matches!(reason, DeclarationFreshnessReason::CoveredCommitDiffers)
    })
}
