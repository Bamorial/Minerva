use crate::{ProjectRepository, RebuildResult, TaskRepository};
use minerva_domain::MinervaError;
use std::path::Path;

pub struct RebuildService;

impl RebuildService {
    pub fn run(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        dry_run: bool,
    ) -> Result<RebuildResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        task_repo.rebuild_derived_state(&root, dry_run)
    }
}
