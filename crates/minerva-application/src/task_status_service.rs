use crate::{ProjectRepository, TaskRepository, TaskShowOptions, TaskShowService};
use minerva_domain::MinervaError;
use std::path::Path;

pub struct TaskStatusService;

impl TaskStatusService {
    pub fn show(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        task_ref: &str,
    ) -> Result<String, MinervaError> {
        TaskShowService::show(
            project_repo,
            task_repo,
            start,
            task_ref,
            &TaskShowOptions::default(),
        )
        .map(|result| result.render())
    }
}
