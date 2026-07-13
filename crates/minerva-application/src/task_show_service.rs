use crate::{ProjectRepository, TaskRepository, git_support::git_head};
use humantime::format_rfc3339;
use minerva_domain::{
    DeclarationFreshness, DeclarationFreshnessReason, MinervaError, Relationship,
    RelationshipType, Task, TaskId,
};
use std::{collections::BTreeMap, path::Path, time::SystemTime};

use crate::{
    TaskShowFreshness, TaskShowLink, TaskShowOptions, TaskShowRelationship,
    TaskShowResult, TaskShowTimestamps,
};

pub struct TaskShowService;

impl TaskShowService {
    pub fn show(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        task_ref: &str,
        options: &TaskShowOptions,
    ) -> Result<TaskShowResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let task = task_repo.resolve_task(&root, task_ref)?;
        let all = task_repo
            .list_tasks(&root)?
            .into_iter()
            .map(|task| (task.id, task))
            .collect::<BTreeMap<_, _>>();
        let outgoing = task_repo.list_relationships_from(&root, task.id)?;
        let incoming = task_repo.list_relationships_to(&root, task.id)?;
        let mut probe = task_repo.read_declaration_freshness(&root, task.id)?;
        probe.current_commit_hash = git_head(&root);
        Ok(TaskShowResult {
            parent: task.parent_id.and_then(|id| link(&all, id)),
            dependencies: dependencies(task.id, &outgoing, &incoming, &all),
            relationships: relationships(task.id, outgoing, incoming, &all),
            freshness: freshness(&probe.evaluate()),
            timestamps: timestamps(&task),
            instructions: options
                .include_instructions
                .then(|| task_repo.read_task_instructions(&root, task.id))
                .transpose()?,
            declaration: options
                .include_declaration
                .then(|| task_repo.read_task_declaration(&root, task.id))
                .transpose()?,
            task,
        })
    }
}

fn dependencies(
    task_id: TaskId,
    outgoing: &[Relationship],
    incoming: &[Relationship],
    all: &BTreeMap<TaskId, Task>,
) -> Vec<TaskShowLink> {
    outgoing
        .iter()
        .filter(|rel| {
            rel.relationship_type == RelationshipType::DependsOn
                && rel.source_task == task_id
        })
        .filter_map(|rel| link(all, rel.target_task))
        .chain(
            incoming
                .iter()
                .filter(|rel| {
                    rel.relationship_type == RelationshipType::Blocks
                        && rel.target_task == task_id
                })
                .filter_map(|rel| link(all, rel.source_task)),
        )
        .collect()
}

fn relationships(
    _task_id: TaskId,
    outgoing: Vec<Relationship>,
    incoming: Vec<Relationship>,
    all: &BTreeMap<TaskId, Task>,
) -> Vec<TaskShowRelationship> {
    outgoing
        .into_iter()
        .filter(|rel| {
            !matches!(
                rel.relationship_type,
                RelationshipType::DependsOn | RelationshipType::Parent
            )
        })
        .filter_map(|rel| {
            relation(
                "outgoing",
                rel.relationship_type,
                rel.target_task,
                rel.reason,
                rel.created_at,
                all,
            )
        })
        .chain(
            incoming
                .into_iter()
                .filter(|rel| {
                    !matches!(
                        rel.relationship_type,
                        RelationshipType::Blocks | RelationshipType::Parent
                    )
                })
                .filter_map(|rel| {
                    relation(
                        "incoming",
                        rel.relationship_type,
                        rel.source_task,
                        rel.reason,
                        rel.created_at,
                        all,
                    )
                }),
        )
        .collect::<Vec<_>>()
}

fn relation(
    direction: &str,
    kind: RelationshipType,
    task_id: TaskId,
    reason: Option<String>,
    created_at: SystemTime,
    all: &BTreeMap<TaskId, Task>,
) -> Option<TaskShowRelationship> {
    Some(TaskShowRelationship {
        kind: relationship(kind).into(),
        direction: direction.into(),
        task: link(all, task_id)?,
        reason,
        created_at: stamp(created_at),
    })
}

fn link(all: &BTreeMap<TaskId, Task>, task_id: TaskId) -> Option<TaskShowLink> {
    all.get(&task_id)
        .map(|task| TaskShowLink { id: task.id.to_string(), title: task.title.clone() })
}

fn freshness(value: &minerva_domain::DeclarationFreshnessReport) -> TaskShowFreshness {
    TaskShowFreshness {
        status: status(value.status).into(),
        reasons: value.reasons.iter().map(|value| reason(*value).into()).collect(),
    }
}

fn timestamps(task: &Task) -> TaskShowTimestamps {
    TaskShowTimestamps {
        created_at: stamp(task.created_at),
        updated_at: stamp(task.updated_at),
        completed_at: task.completed_at.map(stamp),
        declaration_updated_at: stamp(task.declaration.updated_at),
    }
}

fn stamp(value: SystemTime) -> String {
    format_rfc3339(value).to_string()
}
fn status(value: DeclarationFreshness) -> &'static str {
    match value {
        DeclarationFreshness::Fresh => "fresh",
        DeclarationFreshness::PotentiallyStale => "potentially-stale",
        DeclarationFreshness::Stale => "stale",
        DeclarationFreshness::Unknown => "unknown",
    }
}
fn relationship(value: RelationshipType) -> &'static str {
    match value {
        RelationshipType::Parent => "parent",
        RelationshipType::DependsOn => "depends-on",
        RelationshipType::Blocks => "blocks",
        RelationshipType::RelatedTo => "related-to",
        RelationshipType::Duplicates => "duplicates",
        RelationshipType::Implements => "implements",
        RelationshipType::References => "references",
    }
}
fn reason(value: DeclarationFreshnessReason) -> &'static str {
    match value {
        DeclarationFreshnessReason::MissingCoveredCommit => "missing-covered-commit",
        DeclarationFreshnessReason::CoveredCommitUnavailable => {
            "covered-commit-unavailable"
        }
        DeclarationFreshnessReason::CoveredCommitDiffers => "covered-commit-differs",
        DeclarationFreshnessReason::InstructionsUpdatedAfterDeclaration => {
            "instructions-updated-after-declaration"
        }
        DeclarationFreshnessReason::RelationshipsUpdatedAfterDeclaration => {
            "relationships-updated-after-declaration"
        }
        DeclarationFreshnessReason::TaskMetadataUpdatedAfterDeclaration => {
            "task-metadata-updated-after-declaration"
        }
    }
}
