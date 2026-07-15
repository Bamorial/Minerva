use crate::app_context;
use minerva_application::{
    CreateTaskRequest, MoveTaskRequest, ProjectInstructionService, ProjectRepository,
    TaskCreationResult, TaskCreationService, TaskDeletionResult, TaskDeletionService,
    TaskInstructionService, TaskMovementService, TaskRelationshipService,
    TaskRepository, TaskShowOptions, TaskShowResult, TaskShowService, TaskStatusResult,
    TaskStatusService, TaskTreeOptions, TaskTreeResult, TaskTreeService,
};
use minerva_domain::{
    AgentPromptMode, MinervaError, Project, Relationship, RelationshipType, StatusKey,
    TaskId,
};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::path::Path;

pub fn load_tree(start: &Path) -> Result<TaskTreeResult, MinervaError> {
    TaskTreeService::tree(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        &TaskTreeOptions {
            status: None,
            archive_state: minerva_application::TaskListArchiveFilter::Active,
        },
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

pub fn load_task_types(start: &Path) -> Result<Vec<String>, MinervaError> {
    let root = FilesystemProjectRepository.locate_project_root(start)?;
    FilesystemProjectRepository
        .load_task_types(&root)
        .map(|types| types.into_iter().map(|item| item.name.to_string()).collect())
}

pub fn load_prompt_mode(start: &Path) -> Result<AgentPromptMode, MinervaError> {
    let root = FilesystemProjectRepository.locate_project_root(start)?;
    FilesystemProjectRepository
        .load_project_config(&root)
        .map(|config| config.agent_prompt_mode)
}

pub fn project_root(start: &Path) -> Result<std::path::PathBuf, MinervaError> {
    FilesystemProjectRepository.locate_project_root(start)
}

pub fn create_task(
    start: &Path,
    title: String,
    task_type: String,
    parent_id: Option<TaskId>,
) -> Result<TaskCreationResult, MinervaError> {
    TaskCreationService::create(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        CreateTaskRequest {
            title,
            task_type: Some(task_type.parse()?),
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

pub fn edit_project_instructions(
    start: &Path,
) -> Result<std::path::PathBuf, MinervaError> {
    ProjectInstructionService::edit(&FilesystemProjectRepository, start)
}

pub fn set_prompt_mode(
    start: &Path,
    mode: AgentPromptMode,
) -> Result<(), MinervaError> {
    let root = FilesystemProjectRepository.locate_project_root(start)?;
    let mut config = FilesystemProjectRepository.load_project_config(&root)?;
    config.agent_prompt_mode = mode;
    FilesystemProjectRepository.save_project_config(&root, &config)
}

pub fn load_context(
    start: &Path,
    task_ref: &str,
    mode: AgentPromptMode,
) -> Result<String, MinervaError> {
    app_context::load(start, task_ref, mode)
}

pub fn add_relationship(
    start: &Path,
    task_ref: &str,
    related_ref: &str,
    relationship_type: RelationshipType,
) -> Result<Relationship, MinervaError> {
    let source = FilesystemTaskRepository.resolve_task(start, task_ref)?;
    let target = FilesystemTaskRepository.resolve_task(start, related_ref)?;
    TaskRelationshipService::create(
        &FilesystemTaskRepository,
        start,
        source.id,
        target.id,
        relationship_type,
        None,
    )
}

pub fn delete_task(
    start: &Path,
    task_ref: &str,
) -> Result<TaskDeletionResult, MinervaError> {
    TaskDeletionService::delete(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        start,
        task_ref,
    )
}

pub fn edit_task_instructions(
    start: &Path,
    task_ref: &str,
) -> Result<std::path::PathBuf, MinervaError> {
    edit_instructions(start, task_ref)
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
