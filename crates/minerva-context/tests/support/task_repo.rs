#![allow(dead_code)]

use minerva_application::{TaskCreateRecord, TaskRepository};
use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationDocument, DeclarationMetadata,
    StatusKey, Task, TaskFacts, TaskId, TaskIdAllocator, TaskPriority, TaskResources,
    TaskTypeKey, TaskVersion,
};
use minerva_storage::FilesystemTaskRepository;
use std::collections::BTreeSet;
use std::path::Path;
use std::time::UNIX_EPOCH;

pub fn persist_task(
    root: &Path,
    sequence: u32,
    title: &str,
    parent_id: Option<TaskId>,
    instructions: &str,
    declaration: &str,
    checks: &[&str],
) -> Task {
    let task = build_task(sequence, title, parent_id, checks);
    FilesystemTaskRepository
        .create_task(
            root,
            &TaskCreateRecord {
                task: task.clone(),
                instructions: instructions.into(),
                declaration: valid_declaration(declaration),
            },
        )
        .unwrap();
    task
}

fn build_task(
    sequence: u32,
    title: &str,
    parent_id: Option<TaskId>,
    checks: &[&str],
) -> Task {
    Task::new(Task {
        schema_version: 1,
        id: TaskIdAllocator::new(sequence - 1).next_id(),
        title: title.into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("backlog").unwrap(),
        parent_id,
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
            files: vec!["src/lib.rs".into()],
            migrations_required: false,
            feature_flags: vec!["context".into()],
            acceptance_checks: checks.iter().map(|value| (*value).into()).collect(),
            resources: TaskResources::default(),
        },
        archive_state: ArchiveState::Active,
    })
    .unwrap()
}

fn valid_declaration(body: &str) -> String {
    DeclarationDocument::template()
        .replace("## Objective\n", &format!("## Objective\n{body}\n"))
}
