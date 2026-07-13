use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValidationSeverity {
    Error,
    Warning,
    Information,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectValidationFinding {
    pub severity: ValidationSeverity,
    pub code: String,
    pub path: String,
    pub task_ref: Option<String>,
    pub message: String,
}
