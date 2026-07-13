use crate::{ProjectRepository, TaskRepository, TaskStatusResult, TaskWriteResult};
use minerva_domain::{MinervaError, Project, StatusKey, Task, TaskTransitionService};
use std::path::Path;

pub struct TaskStatusService;

impl TaskStatusService {
    pub fn set(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        root: &Path,
        task_ref: &str,
        status: StatusKey,
    ) -> Result<TaskStatusResult, MinervaError> {
        let project = project_repo.load_project(root)?;
        let task = task_repo.resolve_task(root, task_ref)?;
        Self::apply(&project, task_repo, root, &task, status, false)
    }

    pub(crate) fn apply(
        project: &Project,
        task_repo: &impl TaskRepository,
        root: &Path,
        task: &Task,
        status: StatusKey,
        completion_override: bool,
    ) -> Result<TaskStatusResult, MinervaError> {
        let next = TaskTransitionService::apply(
            project,
            task,
            status,
            std::time::SystemTime::now(),
        )?;
        if !next.changed {
            let current_version = next.current.version;
            return Ok(TaskStatusResult {
                task: next.current,
                write_result: TaskWriteResult {
                    previous_version: Some(next.previous.version),
                    current_version,
                    event_id: None,
                },
            });
        }
        let write_result =
            task_repo.transition_task(root, &next.current, completion_override)?;
        Ok(TaskStatusResult { task: next.current, write_result })
    }
}
