use crate::{
    list_args::{ArchiveStateArg, ListArgs, SortArg},
    response::CommandOutput,
};
use minerva_application::{
    ProjectRepository, TaskListArchiveFilter, TaskListOptions, TaskListService,
    TaskListSort, TaskRepository,
};
use minerva_domain::{MinervaError, StatusKey, TaskTag, TaskTypeKey};
use std::path::Path;

pub fn execute(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &ListArgs,
) -> Result<CommandOutput, MinervaError> {
    TaskListService::list(project_repo, task_repo, root, &options(args)?)
        .map(crate::list_output::render)
}

fn options(args: &ListArgs) -> Result<TaskListOptions, MinervaError> {
    Ok(TaskListOptions {
        status: args.status.as_deref().map(StatusKey::new).transpose()?,
        task_type: args.task_type.as_deref().map(TaskTypeKey::new).transpose()?,
        parent_ref: args.parent.clone(),
        tag: args.tag.as_deref().map(TaskTag::new).transpose()?,
        archive_state: archive(args.archive_state),
        search: args.search.clone(),
        sort: sort(args.sort),
        offset: args.offset,
        limit: (!args.all).then_some(args.limit),
    })
}

const fn archive(value: ArchiveStateArg) -> TaskListArchiveFilter {
    match value {
        ArchiveStateArg::Active => TaskListArchiveFilter::Active,
        ArchiveStateArg::Archived => TaskListArchiveFilter::Archived,
        ArchiveStateArg::All => TaskListArchiveFilter::All,
    }
}

const fn sort(value: SortArg) -> TaskListSort {
    match value {
        SortArg::Created => TaskListSort::Created,
        SortArg::Updated => TaskListSort::Updated,
        SortArg::Priority => TaskListSort::Priority,
        SortArg::Title => TaskListSort::Title,
        SortArg::Id => TaskListSort::Id,
    }
}
