use crate::{
    cli::{DependArgs, RelateArgs, UnrelateArgs},
    relationship_output,
    response::CommandOutput,
};
use minerva_application::{ProjectRepository, TaskRelationshipService, TaskRepository};
use minerva_domain::{MinervaError, RelationshipType};
use std::path::Path;

pub fn depend(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &DependArgs,
) -> Result<CommandOutput, MinervaError> {
    create(
        project_repo,
        task_repo,
        root,
        &args.task_ref,
        &args.depends_on_ref,
        RelationshipType::DependsOn,
        args.reason.clone(),
    )
}

pub fn undepend(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &DependArgs,
) -> Result<CommandOutput, MinervaError> {
    remove(
        project_repo,
        task_repo,
        root,
        &args.task_ref,
        &args.depends_on_ref,
        RelationshipType::DependsOn,
    )
}

pub fn relate(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &RelateArgs,
) -> Result<CommandOutput, MinervaError> {
    create(
        project_repo,
        task_repo,
        root,
        &args.source_ref,
        &args.target_ref,
        parse_general(&args.relationship_type)?,
        args.reason.clone(),
    )
}

pub fn unrelate(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &UnrelateArgs,
) -> Result<CommandOutput, MinervaError> {
    remove(
        project_repo,
        task_repo,
        root,
        &args.source_ref,
        &args.target_ref,
        parse_general(&args.relationship_type)?,
    )
}

fn create(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    source_ref: &str,
    target_ref: &str,
    relationship_type: RelationshipType,
    reason: Option<String>,
) -> Result<CommandOutput, MinervaError> {
    let root = project_repo.locate_project_root(root)?;
    let source_task = task_repo.resolve_task(&root, source_ref)?;
    let target_task = task_repo.resolve_task(&root, target_ref)?;
    TaskRelationshipService::create(
        task_repo,
        &root,
        source_task.id,
        target_task.id,
        relationship_type,
        reason,
    )
    .map(|relationship| relationship_output::created(&relationship))
}

fn remove(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    source_ref: &str,
    target_ref: &str,
    relationship_type: RelationshipType,
) -> Result<CommandOutput, MinervaError> {
    let root = project_repo.locate_project_root(root)?;
    let source_task = task_repo.resolve_task(&root, source_ref)?;
    let target_task = task_repo.resolve_task(&root, target_ref)?;
    TaskRelationshipService::remove(
        task_repo,
        &root,
        source_task.id,
        target_task.id,
        relationship_type,
    )
    .map(|relationship| relationship_output::removed(&relationship))
}

fn parse_general(value: &str) -> Result<RelationshipType, MinervaError> {
    match value.replace('-', "_").as_str() {
        "related_to" => Ok(RelationshipType::RelatedTo),
        "duplicates" => Ok(RelationshipType::Duplicates),
        "implements" => Ok(RelationshipType::Implements),
        "references" => Ok(RelationshipType::References),
        _ => Err(MinervaError::InvalidConfiguration {
            key: "relationship_type".into(),
            reason: "must be one of related_to, duplicates, implements, references"
                .into(),
        }),
    }
}
