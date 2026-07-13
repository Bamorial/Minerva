use minerva_domain::{TaskId, TaskPriority, TaskTag, TaskTypeKey};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTaskRequest {
    pub title: String,
    pub task_type: Option<TaskTypeKey>,
    pub parent_id: Option<TaskId>,
    pub priority: Option<TaskPriority>,
    pub tags: Option<BTreeSet<TaskTag>>,
}
