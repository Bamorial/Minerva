use minerva_application::{
    ProjectRepository, TaskListArchiveFilter, TaskShowOptions, TaskShowResult,
    TaskShowService, TaskTreeOptions, TaskTreeResult, TaskTreeService,
};
use minerva_domain::MinervaError;
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::path::Path;

pub fn load_tree(start: &Path) -> Result<TaskTreeResult, MinervaError> {
    TaskTreeService::tree(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        &TaskTreeOptions { status: None, archive_state: TaskListArchiveFilter::All },
    )
}

pub fn load_task(start: &Path, task_ref: &str) -> Result<TaskShowResult, MinervaError> {
    TaskShowService::show(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        task_ref,
        &TaskShowOptions::default(),
    )
}

pub fn project_root(start: &Path) -> Result<std::path::PathBuf, MinervaError> {
    FilesystemProjectRepository.locate_project_root(start)
}
