use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, StatusKey, Task, TaskFacts,
    TaskId, TaskIdAllocator, TaskPriority, TaskTypeKey, TaskVersion,
};
use std::{collections::BTreeSet, time::UNIX_EPOCH};

pub fn sample_task(title: &str, parent_id: Option<TaskId>, archived: bool) -> Task {
    Task::new(Task {
        schema_version: 1,
        id: TaskIdAllocator::new(title.len() as u32).next_id(),
        title: title.into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("backlog").unwrap(),
        parent_id,
        priority: TaskPriority::Medium,
        tags: BTreeSet::new(),
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
        facts: TaskFacts::default(),
        archive_state: if archived {
            ArchiveState::Archived
        } else {
            ArchiveState::Active
        },
    })
    .unwrap()
}
