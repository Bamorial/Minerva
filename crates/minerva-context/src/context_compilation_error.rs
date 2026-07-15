use crate::ContextBudgetError;
use minerva_domain::{DeclarationFreshnessReason, MinervaError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextCompilationError {
    Minerva(MinervaError),
    Budget(ContextBudgetError),
    StaleReference { task_ref: String, reasons: Vec<DeclarationFreshnessReason> },
}

impl From<MinervaError> for ContextCompilationError {
    fn from(value: MinervaError) -> Self {
        Self::Minerva(value)
    }
}

impl From<ContextBudgetError> for ContextCompilationError {
    fn from(value: ContextBudgetError) -> Self {
        Self::Budget(value)
    }
}

impl std::fmt::Display for ContextCompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minerva(err) => err.fmt(f),
            Self::Budget(err) => err.fmt(f),
            Self::StaleReference { task_ref, reasons } => write!(
                f,
                "task `{task_ref}` has a stale declaration: {}",
                reasons
                    .iter()
                    .map(|value| reason(*value))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl std::error::Error for ContextCompilationError {}

fn reason(value: DeclarationFreshnessReason) -> &'static str {
    match value {
        DeclarationFreshnessReason::MissingCoveredCommit => "missing-covered-commit",
        DeclarationFreshnessReason::CoveredCommitUnavailable => {
            "covered-commit-unavailable"
        }
        DeclarationFreshnessReason::CoveredCommitDiffers => "covered-commit-differs",
    }
}
