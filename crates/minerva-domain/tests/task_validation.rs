use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, MinervaError, StatusKey, Task,
    TaskFacts, TaskIdAllocator, TaskPriority, TaskTypeKey, TaskVersion,
};
use std::{collections::BTreeSet, time::UNIX_EPOCH};

#[test]
fn task_rejects_empty_titles_and_missing_completed_timestamps() {
    let empty = Task::new(task("  ", "in-progress", None, TaskVersion::initial()));
    let self_parent = Task::new(Task {
        parent_id: Some(TaskIdAllocator::new(0).next_id()),
        ..task("Ship feature", "in-progress", None, TaskVersion::initial())
    });
    let missing_completed_at =
        Task::new(task("Ship feature", "completed", None, TaskVersion::initial()));
    assert!(
        matches!(empty, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "title")
    );
    assert!(
        matches!(self_parent, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "parent_id")
    );
    assert!(
        matches!(missing_completed_at, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "completed_at")
    );
}

#[test]
fn task_successors_must_preserve_identity_and_increment_version() {
    let current =
        Task::new(task("Ship feature", "in-progress", None, TaskVersion::initial()))
            .unwrap();
    let next = Task {
        updated_at: UNIX_EPOCH,
        version: current.version.next(),
        ..current.clone()
    };
    let skipped = Task { version: TaskVersion::new(3).unwrap(), ..current.clone() };
    assert!(next.validate_successor(&current).is_ok());
    assert!(
        matches!(skipped.validate_successor(&current), Err(MinervaError::InvalidConfiguration { key, .. }) if key == "version")
    );
}

#[test]
fn task_rejects_blank_fact_values() {
    let mut task = task("Ship feature", "in-progress", None, TaskVersion::initial());
    task.facts.modules.push(" ".into());
    let result = Task::new(task);
    assert!(
        matches!(result, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "facts.modules")
    );
}

fn task(
    title: &str,
    status: &str,
    completed_at: Option<std::time::SystemTime>,
    version: TaskVersion,
) -> Task {
    Task {
        schema_version: 1,
        id: TaskIdAllocator::new(0).next_id(),
        title: title.into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new(status).unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: BTreeSet::new(),
        created_at: UNIX_EPOCH,
        updated_at: UNIX_EPOCH,
        completed_at,
        version,
        declaration: DeclarationMetadata {
            version: 1,
            updated_at: UNIX_EPOCH,
            updated_by: DeclarationActor::Human,
            commit_hash: None,
        },
        facts: TaskFacts::default(),
        archive_state: ArchiveState::Active,
    }
}
