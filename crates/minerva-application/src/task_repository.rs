use minerva_domain::{MinervaError, Task, TaskId, TaskVersion};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskWriteResult {
    pub previous_version: Option<TaskVersion>,
    pub current_version: TaskVersion,
}

pub trait TaskRepository {
    fn create_task(
        &self,
        root: &Path,
        task: &Task,
    ) -> Result<TaskWriteResult, MinervaError>;
    fn read_task(&self, root: &Path, task_id: TaskId) -> Result<Task, MinervaError>;
    fn update_task(
        &self,
        root: &Path,
        task: &Task,
    ) -> Result<TaskWriteResult, MinervaError>;
    fn list_tasks(&self, root: &Path) -> Result<Vec<Task>, MinervaError>;
    fn archive_task(
        &self,
        root: &Path,
        task_id: TaskId,
        version: TaskVersion,
    ) -> Result<TaskWriteResult, MinervaError>;
    fn resolve_task(&self, root: &Path, task_ref: &str) -> Result<Task, MinervaError>;
    fn search_tasks(&self, root: &Path, query: &str)
    -> Result<Vec<Task>, MinervaError>;
}
