use crate::{cli::MoveArgs, response::CommandOutput};
use minerva_application::{
    MoveTaskRequest, ProjectRepository, TaskMovementService, TaskRepository,
};
use minerva_domain::MinervaError;
use serde_json::json;
use std::path::Path;

pub fn execute(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &MoveArgs,
) -> Result<CommandOutput, MinervaError> {
    let root = project_repo.locate_project_root(root)?;
    let task = task_repo.resolve_task(&root, &args.task_ref)?;
    let new_parent_id = match &args.parent {
        Some(parent_ref) => Some(task_repo.resolve_task(&root, parent_ref)?.id),
        None => None,
    };
    let result = TaskMovementService::move_task(
        task_repo,
        &root,
        MoveTaskRequest { task_id: task.id, new_parent_id, version: task.version },
    )?;
    let summary = new_parent_id
        .map_or_else(|| "to root".into(), |parent_id| format!("under {parent_id}"));
    Ok(CommandOutput::with_json(
        format!("{} moved {summary} (v{})", result.task.id, result.task.version.get()),
        json!({
            "task": {
                "id": result.task.id,
                "parent_id": result.task.parent_id,
                "version": result.task.version.get(),
            },
            "write": {
                "previous_version": result.write_result.previous_version.map(|v| v.get()),
                "current_version": result.write_result.current_version.get(),
                "event_id": result.write_result.event_id,
            }
        }),
    ))
}
