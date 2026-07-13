use minerva_domain::{
    ArchiveState, ContextPolicy, DeclarationActor, DeclarationMetadata, Project,
    ProjectId, StatusDefinition, StatusKey, StatusTransition, Task, TaskFacts,
    TaskIdAllocator, TaskPriority, TaskTypeKey, TaskVersion,
};
use std::{collections::BTreeSet, time::UNIX_EPOCH};

pub fn project() -> Project {
    let statuses = [
        "backlog",
        "ready",
        "in-progress",
        "blocked",
        "review",
        "completed",
        "cancelled",
    ];
    let defs = statuses.map(|key| {
        StatusDefinition::new(
            StatusKey::new(key).unwrap(),
            key == "completed" || key == "cancelled",
        )
    });
    let edges = [
        ("backlog", "ready"),
        ("ready", "in-progress"),
        ("in-progress", "blocked"),
        ("blocked", "in-progress"),
        ("in-progress", "review"),
        ("review", "completed"),
        ("review", "cancelled"),
        ("completed", "in-progress"),
    ];
    Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: "Minerva".into(),
        created_at: UNIX_EPOCH,
        default_task_type: TaskTypeKey::new("feature").unwrap(),
        default_status: StatusKey::new("backlog").unwrap(),
        statuses: defs.into_iter().collect(),
        transitions: edges
            .into_iter()
            .map(|(from, to)| {
                StatusTransition::new(
                    StatusKey::new(from).unwrap(),
                    StatusKey::new(to).unwrap(),
                )
            })
            .collect(),
        context_policy: ContextPolicy::strict(),
    })
    .unwrap()
}

pub fn task(status: &str, completed_at: Option<std::time::SystemTime>) -> Task {
    Task::new(Task {
        schema_version: 1,
        id: TaskIdAllocator::new(0).next_id(),
        title: "Define task transitions".into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new(status).unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: BTreeSet::new(),
        created_at: UNIX_EPOCH,
        updated_at: UNIX_EPOCH,
        completed_at,
        version: TaskVersion::initial(),
        declaration: DeclarationMetadata {
            version: 1,
            updated_at: UNIX_EPOCH,
            updated_by: DeclarationActor::Human,
            commit_hash: None,
        },
        facts: TaskFacts::default(),
        archive_state: ArchiveState::Active,
    })
    .unwrap()
}
