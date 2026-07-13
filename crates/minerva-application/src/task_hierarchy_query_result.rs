use minerva_domain::Task;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskHierarchyQueryResult {
    pub task: Task,
    pub items: Vec<Task>,
}
