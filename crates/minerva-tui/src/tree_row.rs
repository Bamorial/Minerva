use minerva_domain::{Task, TaskId};

#[derive(Debug, Clone)]
pub struct TreeRow {
    pub task: Task,
    pub depth: usize,
    pub parent_id: Option<TaskId>,
    pub has_children: bool,
    pub expanded: bool,
}

impl TreeRow {
    #[must_use]
    pub fn new(
        task: &Task,
        depth: usize,
        parent_id: Option<TaskId>,
        has_children: bool,
        expanded: bool,
    ) -> Self {
        Self { task: task.clone(), depth, parent_id, has_children, expanded }
    }
}
