use crate::{cli::NewArgs, new_prompt, new_resolve, response::CommandOutput};
use minerva_application::{
    CreateTaskRequest, EditorLauncher, ProjectRepository, TaskCreationService,
    TaskRepository,
};
use minerva_domain::{MinervaError, Task};
use serde_json::json;
use std::path::Path;

pub fn execute(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    start: &Path,
    args: &NewArgs,
) -> Result<CommandOutput, MinervaError> {
    let root = project_repo.locate_project_root(start)?;
    let config = project_repo.load_project_config(&root)?;
    let project = project_repo.load_project(&root)?;
    let task_types = project_repo.load_task_types(&root)?;
    let title = match &args.title {
        Some(title) => title.clone(),
        None => new_prompt::prompt("Title: ")?,
    };
    let task_type = new_resolve::task_type(
        args.task_type.as_deref(),
        &task_types,
        &project.default_task_type,
    )?;
    let parent_id = new_resolve::parent(task_repo, &root, args.parent.as_deref())?;
    let priority = new_resolve::priority(args.priority.as_deref())?;
    let tags = new_resolve::tags(&args.tags)?;
    let result = TaskCreationService::create(
        project_repo,
        task_repo,
        &root,
        CreateTaskRequest { title, task_type, parent_id, priority, tags },
    )?;
    let path = task_repo.prepare_task_instructions(&root, result.task.id)?;
    let edited =
        edit_new_task(task_repo, &root, &config, &result.task, &path, args.no_edit)?;
    let current = task_repo.read_task(&root, result.task.id)?;
    let text = format!("created {} {}", current.id, current.title);
    let json = json!({
        "task": {
            "id": current.id.to_string(),
            "title": current.title,
            "task_type": current.task_type.to_string(),
            "parent_id": current.parent_id.map(|id| id.to_string()),
            "priority": format!("{:?}", current.priority),
            "tags": current.tags.into_iter().map(new_resolve::tag_string).collect::<Vec<_>>(),
            "status": current.status.as_str(),
            "version": current.version.get(),
            "path": path.parent().unwrap().display().to_string(),
        },
        "instructions": {
            "path": path.display().to_string(),
            "opened": !args.no_edit,
            "edited": edited,
        }
    });
    Ok(CommandOutput::with_json(text, json))
}

fn edit_new_task(
    task_repo: &impl TaskRepository,
    root: &Path,
    config: &minerva_domain::ProjectConfig,
    task: &Task,
    path: &std::path::Path,
    no_edit: bool,
) -> Result<bool, MinervaError> {
    if no_edit {
        return Ok(false);
    }
    let before = task_repo.read_task_instructions(root, task.id)?;
    EditorLauncher::edit_path(path, Some(config))?;
    let after = task_repo.read_task_instructions(root, task.id)?;
    (after != before)
        .then(|| {
            task_repo.update_task_instructions(root, task.id, task.version, &after)
        })
        .transpose()?;
    Ok(after != before)
}
