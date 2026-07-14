use crate::{ProjectMigrationResult, ProjectRepository};
use minerva_domain::MinervaError;
use std::path::Path;

pub struct ProjectMigrationService;

impl ProjectMigrationService {
    pub fn run(
        project_repo: &impl ProjectRepository,
        start: &Path,
        dry_run: bool,
    ) -> Result<ProjectMigrationResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        project_repo.migrate_project_state(&root, dry_run)
    }
}
