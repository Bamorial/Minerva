use crate::{EditorLauncher, ProjectRepository, TaskRepository};
use minerva_domain::MinervaError;
use std::path::{Path, PathBuf};

pub struct TaskInstructionService;

impl TaskInstructionService {
    pub fn edit(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        task_ref: &str,
    ) -> Result<PathBuf, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let config = project_repo.load_project_config(&root)?;
        let task = task_repo.resolve_task(&root, task_ref)?;
        let path = task_repo.prepare_task_instructions(&root, task.id)?;
        let before = task_repo.read_task_instructions(&root, task.id)?;
        EditorLauncher::edit_path(&path, Some(&config))?;
        let after = task_repo.read_task_instructions(&root, task.id)?;
        if after != before {
            task_repo.update_task_instructions(&root, task.id, task.version, &after)?;
        }
        Ok(path)
    }
}
