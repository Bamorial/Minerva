use minerva_domain::Task;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCreateRecord {
    pub task: Task,
    pub instructions: String,
    pub declaration: String,
}
