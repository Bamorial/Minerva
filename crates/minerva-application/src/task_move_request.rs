use minerva_domain::{TaskId, TaskVersion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveTaskRequest {
    pub task_id: TaskId,
    pub new_parent_id: Option<TaskId>,
    pub version: TaskVersion,
}
