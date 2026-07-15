use minerva_application::{ProjectRepository, TaskRepository};
use minerva_context::{ContextCompilationRequest, ContextCompilationService};
use minerva_domain::{AgentPromptMode, MinervaError, Task};
use minerva_storage::{
    FilesystemProjectRepository, FilesystemTaskRepository, MinervaLayout,
};
use std::path::Path;

pub fn load(
    start: &Path,
    task_ref: &str,
    mode: AgentPromptMode,
) -> Result<String, MinervaError> {
    match mode {
        AgentPromptMode::Static => static_prompt(start, task_ref),
        AgentPromptMode::Exploration => exploration_prompt(start, task_ref),
    }
}

fn static_prompt(start: &Path, task_ref: &str) -> Result<String, MinervaError> {
    ContextCompilationService::compile(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        &ContextCompilationRequest::new(task_ref),
    )
    .map(|result| format!("[static]\n\n{}", result.markdown))
    .map_err(super::app_services::map_context_error)
}

fn exploration_prompt(start: &Path, task_ref: &str) -> Result<String, MinervaError> {
    let root = FilesystemProjectRepository.locate_project_root(start)?;
    let task = FilesystemTaskRepository.resolve_task(&root, task_ref)?;
    let tasks = FilesystemTaskRepository.list_tasks(&root)?;
    let layout = MinervaLayout::new(&root);
    let siblings = sibling_paths(&layout, &tasks, &task);
    Ok(format!(
        "[exploration]\n\nInvestigate the referenced Minerva files before changing code.\nTask path: `{}`\nProject instructions: `{}`\nParent path: {}\nSibling paths:\n{}\nTask instructions: `{}`\nDeclaration to complete: `{}`",
        layout.task_dir(task.id).display(),
        layout.instructions_file().display(),
        parent_path(&layout, &task),
        sibling_block(&siblings),
        layout.task_instructions_file(task.id).display(),
        layout.declaration_file(task.id).display(),
    ))
}

fn parent_path(layout: &MinervaLayout, task: &Task) -> String {
    task.parent_id.map_or_else(
        || "none".into(),
        |id| format!("`{}`", layout.task_dir(id).display()),
    )
}

fn sibling_paths(layout: &MinervaLayout, tasks: &[Task], task: &Task) -> Vec<String> {
    tasks
        .iter()
        .filter(|item| item.parent_id == task.parent_id && item.id != task.id)
        .map(|item| format!("- `{}`", layout.task_dir(item.id).display()))
        .collect()
}

fn sibling_block(paths: &[String]) -> String {
    if paths.is_empty() { "- none".into() } else { paths.join("\n") }
}
