use crate::{list_args::ArchiveStateArg, response::CommandOutput, tree_args::TreeArgs};
use minerva_application::{
    ProjectRepository, TaskListArchiveFilter, TaskRepository, TaskTreeOptions,
    TaskTreeService,
};
use minerva_domain::{MinervaError, StatusKey};
use std::path::Path;

pub fn execute(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &TreeArgs,
) -> Result<CommandOutput, MinervaError> {
    TaskTreeService::tree(project_repo, task_repo, root, &options(args)?)
        .map(|result| crate::tree_output::render(&result))
}

fn options(args: &TreeArgs) -> Result<TaskTreeOptions, MinervaError> {
    Ok(TaskTreeOptions {
        status: args.status.as_deref().map(StatusKey::new).transpose()?,
        archive_state: archive(args.archive_state),
    })
}

const fn archive(value: ArchiveStateArg) -> TaskListArchiveFilter {
    match value {
        ArchiveStateArg::Active => TaskListArchiveFilter::Active,
        ArchiveStateArg::Archived => TaskListArchiveFilter::Archived,
        ArchiveStateArg::All => TaskListArchiveFilter::All,
    }
}
