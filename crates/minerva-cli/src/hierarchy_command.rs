use crate::{hierarchy_output, response::CommandOutput};
use minerva_application::{
    ProjectRepository, TaskHierarchyQueryService, TaskRepository,
};
use minerva_domain::MinervaError;
use std::path::Path;

pub fn children(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    task_ref: &str,
) -> Result<CommandOutput, MinervaError> {
    let root = project_repo.locate_project_root(root)?;
    TaskHierarchyQueryService::children(task_repo, &root, task_ref)
        .map(hierarchy_output::children)
}

pub fn ancestors(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    task_ref: &str,
) -> Result<CommandOutput, MinervaError> {
    let root = project_repo.locate_project_root(root)?;
    TaskHierarchyQueryService::ancestors(task_repo, &root, task_ref)
        .map(hierarchy_output::ancestors)
}
