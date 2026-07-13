use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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
    pub declaration_updated_at: SystemTime,
    pub task_updated_at: SystemTime,
    pub instructions_updated_at: Option<SystemTime>,
    pub relationships_updated_at: Option<SystemTime>,
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
    InstructionsUpdatedAfterDeclaration,
    RelationshipsUpdatedAfterDeclaration,
    TaskMetadataUpdatedAfterDeclaration,
}

impl DeclarationFreshnessProbe {
    #[must_use]
    pub fn evaluate(&self) -> DeclarationFreshnessReport {
        let mut reasons = Vec::new();
        let task_changed = self.task_updated_at > self.declaration_updated_at;
        let instructions_stale = task_changed
            && newer(self.instructions_updated_at, self.declaration_updated_at);
        let relationships_stale =
            newer(self.relationships_updated_at, self.declaration_updated_at);
        if instructions_stale {
            reasons
                .push(DeclarationFreshnessReason::InstructionsUpdatedAfterDeclaration);
        }
        if relationships_stale {
            reasons
                .push(DeclarationFreshnessReason::RelationshipsUpdatedAfterDeclaration);
        }
        if task_changed && !instructions_stale && !relationships_stale {
            reasons
                .push(DeclarationFreshnessReason::TaskMetadataUpdatedAfterDeclaration);
        }
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

fn newer(value: Option<SystemTime>, than: SystemTime) -> bool {
    value.is_some_and(|value| value > than)
}

fn stale(reasons: &[DeclarationFreshnessReason]) -> bool {
    reasons.iter().any(|reason| {
        matches!(
            reason,
            DeclarationFreshnessReason::CoveredCommitDiffers
                | DeclarationFreshnessReason::InstructionsUpdatedAfterDeclaration
                | DeclarationFreshnessReason::RelationshipsUpdatedAfterDeclaration
                | DeclarationFreshnessReason::TaskMetadataUpdatedAfterDeclaration
        )
    })
}
