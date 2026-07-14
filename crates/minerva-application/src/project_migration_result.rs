use crate::ProjectMigrationOperation;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProjectMigrationResult {
    pub start_version: u32,
    pub target_version: u32,
    pub steps: Vec<ProjectMigrationStep>,
}

impl ProjectMigrationResult {
    #[must_use]
    pub fn is_current(&self) -> bool {
        self.steps.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProjectMigrationStep {
    pub name: String,
    pub from_version: u32,
    pub to_version: u32,
    pub operations: Vec<ProjectMigrationOperation>,
}
