use crate::{EditorLauncher, ProjectRepository, TaskRepository};
use minerva_domain::{DeclarationActor, DeclarationDocument, MinervaError};
use std::path::{Path, PathBuf};

pub struct TaskDeclarationService;

impl TaskDeclarationService {
    pub fn edit(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        task_ref: &str,
    ) -> Result<PathBuf, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let config = project_repo.load_project_config(&root)?;
        let task = task_repo.resolve_task(&root, task_ref)?;
        let path = task_repo.prepare_task_declaration(&root, task.id)?;
        let before = task_repo.read_task_declaration(&root, task.id)?;
        EditorLauncher::edit_path(&path, Some(&config))?;
        let after = task_repo.read_task_declaration(&root, task.id)?;
        if DeclarationDocument::content_hash(&after)
            != DeclarationDocument::content_hash(&before)
        {
            task_repo.update_task_declaration(
                &root,
                task.id,
                task.version,
                actor(),
                crate::git_support::git_head(&root),
                &after,
            )?;
        }
        Ok(path)
    }
}

fn actor() -> DeclarationActor {
    std::env::var("MINERVA_AGENT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .and_then(|value| DeclarationActor::agent(value).ok())
        .unwrap_or(DeclarationActor::Human)
}
