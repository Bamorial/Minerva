use crate::TaskWriteResult;
use minerva_domain::Task;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStatusResult {
    pub task: Task,
    pub write_result: TaskWriteResult,
}
