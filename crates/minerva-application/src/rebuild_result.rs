use crate::RebuildAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebuildResult {
    pub index_path: String,
    pub index_action: RebuildAction,
    pub task_errors: Vec<RebuildTaskError>,
}

impl RebuildResult {
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.task_errors.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebuildTaskError {
    pub task_ref: String,
    pub path: String,
    pub reason: String,
}
