use crate::{EditorLauncher, ProjectRepository};
use minerva_domain::MinervaError;
use std::path::{Path, PathBuf};

pub struct ProjectInstructionService;

impl ProjectInstructionService {
    pub fn edit(
        project_repo: &impl ProjectRepository,
        start: &Path,
    ) -> Result<PathBuf, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let config = project_repo.load_project_config(&root)?;
        let path = project_repo.prepare_project_instructions(&root)?;
        EditorLauncher::edit_path(&path, Some(&config))?;
        Ok(path)
    }
}
