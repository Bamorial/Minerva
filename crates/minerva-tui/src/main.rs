use minerva_application::{ProjectRepository, TaskRepository, render_tui};
use minerva_domain::MinervaError;
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};

fn main() {
    match run() {
        Ok(message) => println!("{message}"),
        Err(error) => {
            let message = render_tui(&error);
            eprintln!("{}\n{}", message.title, message.body);
        }
    }
}

fn run() -> Result<String, MinervaError> {
    let root = std::env::current_dir().map_err(|error| io_error(&error))?;
    let project_repo = FilesystemProjectRepository;
    let task_repo = FilesystemTaskRepository;
    let root = project_repo.locate_project_root(&root)?;
    let mut tasks = task_repo.list_tasks(&root)?;
    tasks.sort_by_key(|task| task.id.sequence().get());
    let workspace = minerva_context::compile_workspace_context();
    let task = tasks.first().map_or_else(
        || "Task facts: none".into(),
        minerva_context::compile_task_context,
    );
    Ok(format!("{workspace}\n\n{task}"))
}

fn io_error(error: &std::io::Error) -> MinervaError {
    MinervaError::InvalidConfiguration { key: "cwd".into(), reason: error.to_string() }
}
