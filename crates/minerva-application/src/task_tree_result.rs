use crate::TaskListArchiveFilter;
use minerva_domain::{StatusKey, Task};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTreeOptions {
    pub status: Option<StatusKey>,
    pub archive_state: TaskListArchiveFilter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTreeResult {
    pub roots: Vec<TaskTreeNode>,
    pub total: usize,
    pub matched: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTreeNode {
    pub task: Task,
    pub children: Vec<TaskTreeNode>,
}
