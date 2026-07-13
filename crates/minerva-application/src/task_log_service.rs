use crate::{ProjectRepository, TaskLogResult, TaskRepository};
use minerva_domain::{MinervaError, TaskEventKind};
use std::path::Path;

pub struct TaskLogService;

impl TaskLogService {
    pub fn show(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        task_ref: &str,
        filters: &[TaskEventKind],
    ) -> Result<TaskLogResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let task = task_repo.resolve_task(&root, task_ref)?;
        let mut result = task_repo.read_task_log(&root, task.id)?;
        if !filters.is_empty() {
            result.events.retain(|event| filters.contains(&event.kind));
        }
        Ok(TaskLogResult {
            task,
            filters: filters.to_vec(),
            events: result.events,
            issues: result.issues,
        })
    }
}
