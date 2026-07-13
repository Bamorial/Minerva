use crate::{response::CommandOutput, status_args::StatusArgs};
use minerva_application::{
    CompleteTaskRequest, ProjectRepository, TaskCompletionService, TaskRepository,
    TaskStatusService, TaskWriteResult,
};
use minerva_domain::{MinervaError, StatusKey, Task};
use serde_json::json;
use std::path::Path;

pub fn set(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &StatusArgs,
) -> Result<CommandOutput, MinervaError> {
    let status = StatusKey::new(&args.status)?;
    let result =
        TaskStatusService::set(project_repo, task_repo, root, &args.task_ref, &status)?;
    Ok(render(&result.task, &result.write_result))
}

pub fn complete(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    task_ref: &str,
) -> Result<CommandOutput, MinervaError> {
    let task = task_repo.resolve_task(root, task_ref)?;
    let result = TaskCompletionService::complete(
        project_repo,
        task_repo,
        root,
        CompleteTaskRequest {
            task_id: task.id,
            version: task.version,
            allow_declaration_override: false,
        },
    )?;
    Ok(render(&result.task, &result.write_result))
}

pub fn reopen(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    task_ref: &str,
) -> Result<CommandOutput, MinervaError> {
    let status = StatusKey::new("in-progress").unwrap();
    let result =
        TaskStatusService::set(project_repo, task_repo, root, task_ref, &status)?;
    Ok(render(&result.task, &result.write_result))
}

fn render(task: &Task, write_result: &TaskWriteResult) -> CommandOutput {
    let changed = write_result.event_id.is_some();
    let text = if changed {
        format!("{} -> {} (v{})", task.id, task.status, task.version.get())
    } else {
        format!("{} unchanged at {} (v{})", task.id, task.status, task.version.get())
    };
    CommandOutput::with_json(
        text,
        json!({
            "task": {
                "id": task.id, "status": task.status, "version": task.version.get(),
                "completed_at": task.completed_at.map(humantime::format_rfc3339).map(|v| v.to_string()),
            },
            "changed": changed,
            "write": {
                "previous_version": write_result.previous_version.map(minerva_domain::TaskVersion::get),
                "current_version": write_result.current_version.get(),
                "event_id": write_result.event_id,
            }
        }),
    )
}
