use crate::ProjectMigrationAction;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProjectMigrationOperation {
    pub action: ProjectMigrationAction,
    pub path: String,
    pub backup_path: Option<String>,
    pub message: String,
}
