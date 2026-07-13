#![allow(dead_code)]

use std::{
    env, fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, StatusKey, Task,
    TaskIdAllocator, TaskPriority, TaskTypeKey, TaskVersion,
};

pub fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

pub fn temp_repo(name: &str) -> PathBuf {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = env::temp_dir().join(format!("minerva-storage-{name}-{stamp}"));
    fs::create_dir_all(root.join(".minerva")).unwrap();
    root
}

pub fn sample_task() -> Task {
    Task::new(Task {
        schema_version: 1,
        id: TaskIdAllocator::new(0).next_id(),
        title: "Define task serializer".into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("in-progress").unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: Default::default(),
        created_at: UNIX_EPOCH,
        updated_at: UNIX_EPOCH,
        completed_at: None,
        version: TaskVersion::initial(),
        declaration: DeclarationMetadata {
            version: 1,
            updated_at: UNIX_EPOCH,
            updated_by: DeclarationActor::Human,
            commit_hash: Some("abc123".into()),
        },
        archive_state: ArchiveState::Active,
    })
    .unwrap()
}
