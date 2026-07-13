use crate::{ProjectRepository, ProjectValidationResult, TaskRepository};
use minerva_domain::MinervaError;
use std::path::Path;

pub struct ProjectValidationService;

impl ProjectValidationService {
    pub fn run(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
    ) -> Result<ProjectValidationResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        task_repo.validate_project_state(&root)
    }
}
