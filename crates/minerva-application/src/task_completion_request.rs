use minerva_domain::{TaskId, TaskVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompleteTaskRequest {
    pub task_id: TaskId,
    pub version: TaskVersion,
    pub allow_declaration_override: bool,
}
