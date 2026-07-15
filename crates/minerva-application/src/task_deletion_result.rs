use minerva_domain::{Task, TaskId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDeletionResult {
    pub task: Task,
    pub deleted_task_ids: Vec<TaskId>,
}
