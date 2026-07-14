use crate::{ProjectRepository, RepairResult, TaskRepository};
use minerva_domain::MinervaError;
use std::path::Path;

pub struct RepairService;

impl RepairService {
    pub fn run(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        dry_run: bool,
    ) -> Result<RepairResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let mut result = task_repo.repair_project_state(&root, dry_run)?;
        result.validation =
            if dry_run { None } else { Some(task_repo.validate_project_state(&root)?) };
        Ok(result)
    }
}
