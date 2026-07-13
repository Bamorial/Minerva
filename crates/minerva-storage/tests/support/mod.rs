#![allow(dead_code)]

use std::{
    env, fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use minerva_application::TaskCreateRecord;
use minerva_domain::{
    ArchiveState, ContextPolicy, DeclarationActor, DeclarationDocument,
    DeclarationMetadata, Project, ProjectId, Relationship, RelationshipId,
    RelationshipType, StatusDefinition, StatusKey, StatusTransition, Task, TaskFacts,
    TaskIdAllocator, TaskPriority, TaskSlug, TaskTypeKey, TaskVersion,
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
    task(1, "Define task serializer")
}

pub fn create_record(task: Task) -> TaskCreateRecord {
    TaskCreateRecord {
        task,
        instructions: "# Feature\n".into(),
        declaration: DeclarationDocument::template(),
    }
}

pub fn task(sequence: u32, title: &str) -> Task {
    let allocator = TaskIdAllocator::new(sequence - 1);
    Task::new(Task {
        schema_version: 1,
        id: allocator.next_id(),
        title: title.into(),
        slug: Some(TaskSlug::new(title.to_lowercase().replace(' ', "-")).unwrap()),
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
        facts: TaskFacts::default(),
        archive_state: ArchiveState::Active,
    })
    .unwrap()
}

pub fn sample_project() -> Project {
    Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: "Minerva".into(),
        created_at: UNIX_EPOCH,
        default_task_type: TaskTypeKey::new("feature").unwrap(),
        default_status: StatusKey::new("backlog").unwrap(),
        statuses: vec![
            StatusDefinition::new(StatusKey::new("backlog").unwrap(), false),
            StatusDefinition::new(StatusKey::new("done").unwrap(), true),
        ],
        transitions: vec![StatusTransition::new(
            StatusKey::new("backlog").unwrap(),
            StatusKey::new("done").unwrap(),
        )],
        context_policy: ContextPolicy::new(12, 2, 24).unwrap(),
    })
    .unwrap()
}

pub fn relationship(
    source: minerva_domain::TaskId,
    target: minerva_domain::TaskId,
    relationship_type: RelationshipType,
    reason: Option<&str>,
) -> Relationship {
    Relationship::new(Relationship {
        schema_version: 1,
        id: RelationshipId::new(),
        source_task: source,
        target_task: target,
        relationship_type,
        reason: reason.map(str::to_string),
        created_at: UNIX_EPOCH,
    })
    .unwrap()
}
