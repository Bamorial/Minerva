use crate::response::CommandOutput;
use minerva_application::{ProjectRepository, TaskDeletionService, TaskRepository};
use minerva_domain::MinervaError;
use serde_json::json;
use std::path::Path;

pub fn execute(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    task_ref: &str,
) -> Result<CommandOutput, MinervaError> {
    let result = TaskDeletionService::delete(project_repo, task_repo, root, task_ref)?;
    let text = format!(
        "deleted {} task(s) rooted at {} {}",
        result.deleted_task_ids.len(),
        result.task.id,
        result.task.title
    );
    Ok(CommandOutput::with_json(
        text,
        json!({
            "task": {
                "id": result.task.id.to_string(),
                "title": result.task.title,
            },
            "deleted_task_ids": result
                .deleted_task_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>(),
        }),
    ))
}
