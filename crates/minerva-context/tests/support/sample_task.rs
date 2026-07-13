#![allow(dead_code)]

use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, StatusKey, Task, TaskFacts,
    TaskIdAllocator, TaskPriority, TaskResources, TaskTypeKey, TaskVersion,
};
use std::collections::BTreeSet;
use std::time::UNIX_EPOCH;

#[must_use]
pub fn task() -> Task {
    Task::new(Task {
        schema_version: 1,
        id: TaskIdAllocator::new(0).next_id(),
        title: "Define task facts".into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("backlog").unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: BTreeSet::default(),
        created_at: UNIX_EPOCH,
        updated_at: UNIX_EPOCH,
        completed_at: None,
        version: TaskVersion::initial(),
        declaration: DeclarationMetadata {
            version: 1,
            updated_at: UNIX_EPOCH,
            updated_by: DeclarationActor::Human,
            commit_hash: None,
        },
        facts: TaskFacts {
            modules: vec!["minerva-domain".into()],
            files: vec!["crates/minerva-domain/src/task_facts.rs".into()],
            migrations_required: true,
            feature_flags: vec!["task-facts".into()],
            acceptance_checks: vec!["round-trip persistence".into()],
            resources: TaskResources::default(),
        },
        archive_state: ArchiveState::Active,
    })
    .unwrap()
}
