use minerva_application::{
    CreateTaskRequest, MoveTaskRequest, ProjectRepository, TaskCreationResult,
    TaskCreationService, TaskDeclarationService, TaskInstructionService,
    TaskListArchiveFilter, TaskMovementService, TaskRelationshipService,
    TaskRepository, TaskShowOptions, TaskShowResult, TaskShowService, TaskStatusResult,
    TaskStatusService, TaskTreeOptions, TaskTreeResult, TaskTreeService,
};
use minerva_domain::{
    MinervaError, Project, Relationship, RelationshipType, StatusKey, TaskId,
};
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
        &TaskShowOptions { include_instructions: false, include_declaration: true },
    )
}

pub fn load_project(start: &Path) -> Result<Project, MinervaError> {
    FilesystemProjectRepository.load_project(start)
}

pub fn project_root(start: &Path) -> Result<std::path::PathBuf, MinervaError> {
    FilesystemProjectRepository.locate_project_root(start)
}

pub fn create_task(
    start: &Path,
    title: String,
    parent_id: Option<TaskId>,
) -> Result<TaskCreationResult, MinervaError> {
    TaskCreationService::create(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        CreateTaskRequest {
            title,
            task_type: None,
            parent_id,
            priority: None,
            tags: None,
        },
    )
}

pub fn set_status(
    start: &Path,
    task_ref: &str,
    status: &str,
) -> Result<TaskStatusResult, MinervaError> {
    TaskStatusService::set(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        task_ref,
        &StatusKey::new(status)?,
    )
}

pub fn move_task(
    start: &Path,
    task_ref: &str,
    parent_ref: Option<&str>,
) -> Result<minerva_application::TaskMoveResult, MinervaError> {
    let task = FilesystemTaskRepository.resolve_task(start, task_ref)?;
    let new_parent_id = parent_ref
        .map(|value| FilesystemTaskRepository.resolve_task(start, value))
        .transpose()?
        .map(|task| task.id);
    TaskMovementService::move_task(
        &FilesystemTaskRepository,
        start,
        &MoveTaskRequest { task_id: task.id, new_parent_id, version: task.version },
    )
}

pub fn edit_instructions(
    start: &Path,
    task_ref: &str,
) -> Result<std::path::PathBuf, MinervaError> {
    TaskInstructionService::edit(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        task_ref,
    )
}

pub fn edit_declaration(
    start: &Path,
    task_ref: &str,
) -> Result<std::path::PathBuf, MinervaError> {
    TaskDeclarationService::edit(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        task_ref,
    )
}

pub fn add_dependency(
    start: &Path,
    task_ref: &str,
    depends_on_ref: &str,
) -> Result<Relationship, MinervaError> {
    let source = FilesystemTaskRepository.resolve_task(start, task_ref)?;
    let target = FilesystemTaskRepository.resolve_task(start, depends_on_ref)?;
    TaskRelationshipService::create(
        &FilesystemTaskRepository,
        start,
        source.id,
        target.id,
        RelationshipType::DependsOn,
        None,
    )
}

pub fn remove_dependency(
    start: &Path,
    task_ref: &str,
    depends_on_ref: &str,
) -> Result<Relationship, MinervaError> {
    let source = FilesystemTaskRepository.resolve_task(start, task_ref)?;
    let target = FilesystemTaskRepository.resolve_task(start, depends_on_ref)?;
    TaskRelationshipService::remove(
        &FilesystemTaskRepository,
        start,
        source.id,
        target.id,
        RelationshipType::DependsOn,
    )
}
