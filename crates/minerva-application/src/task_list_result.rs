use minerva_domain::{ArchiveState, StatusKey, Task, TaskTag, TaskTypeKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskListArchiveFilter {
    Active,
    Archived,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskListSort {
    Created,
    Updated,
    Priority,
    Title,
    Id,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskListOptions {
    pub status: Option<StatusKey>,
    pub task_type: Option<TaskTypeKey>,
    pub parent_ref: Option<String>,
    pub tag: Option<TaskTag>,
    pub archive_state: TaskListArchiveFilter,
    pub search: Option<String>,
    pub sort: TaskListSort,
    pub offset: usize,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskListResult {
    pub tasks: Vec<TaskListItem>,
    pub total: usize,
    pub matched: usize,
    pub offset: usize,
    pub limit: Option<usize>,
    pub sort: TaskListSort,
    pub has_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskListItem {
    pub task: Task,
    pub parent: Option<TaskListParent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskListParent {
    pub id: String,
    pub title: String,
}

impl TaskListArchiveFilter {
    #[must_use]
    pub fn matches(self, value: ArchiveState) -> bool {
        matches!(
            (self, value),
            (Self::All, _)
                | (Self::Active, ArchiveState::Active)
                | (Self::Archived, ArchiveState::Archived)
        )
    }
}
