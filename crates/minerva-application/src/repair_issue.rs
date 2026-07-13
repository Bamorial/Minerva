#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RepairIssue {
    pub code: String,
    pub path: String,
    pub task_ref: Option<String>,
    pub message: String,
}
