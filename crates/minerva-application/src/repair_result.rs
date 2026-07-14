use crate::{ProjectValidationResult, RepairIssue, RepairOperation};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RepairResult {
    pub operations: Vec<RepairOperation>,
    pub issues: Vec<RepairIssue>,
    pub validation: Option<ProjectValidationResult>,
}

impl RepairResult {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}
