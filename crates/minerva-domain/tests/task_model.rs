use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, StatusKey, Task,
    TaskIdAllocator, TaskPriority, TaskTag, TaskTypeKey, TaskVersion,
};
use std::{collections::BTreeSet, time::UNIX_EPOCH};

#[test]
fn task_accepts_valid_domain_metadata() {
    let task = Task::new(Task {
        schema_version: 1,
        id: TaskIdAllocator::new(0).next_id(),
        title: "Define task model".into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("in-progress").unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: BTreeSet::from([TaskTag::new("task-model").unwrap()]),
        created_at: UNIX_EPOCH,
        updated_at: UNIX_EPOCH,
        completed_at: None,
        version: TaskVersion::initial(),
        declaration: declaration(3),
        archive_state: ArchiveState::Active,
    })
    .unwrap();
    assert_eq!(task.version, TaskVersion::initial());
}

fn declaration(version: u32) -> DeclarationMetadata {
    DeclarationMetadata {
        version,
        updated_at: UNIX_EPOCH,
        updated_by: DeclarationActor::Human,
        commit_hash: Some("9b12f4".into()),
    }
}
