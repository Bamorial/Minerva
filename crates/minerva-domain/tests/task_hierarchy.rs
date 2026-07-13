use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, MinervaError, StatusKey, Task,
    TaskFacts, TaskIdAllocator, TaskPriority, TaskTypeKey, TaskVersion,
    validate_task_hierarchy,
};
use std::{collections::BTreeSet, time::UNIX_EPOCH};

#[test]
fn hierarchy_rejects_direct_and_indirect_cycles() {
    let ids = TaskIdAllocator::new(0);
    let a = task(ids.next_id(), None);
    let b = task(ids.next_id(), Some(a.id));
    let mut direct = [a.clone(), b.clone()];
    direct[0].parent_id = Some(b.id);
    assert!(matches!(
        validate_task_hierarchy(&direct),
        Err(MinervaError::HierarchyCycle { .. })
    ));
    let c = task(ids.next_id(), Some(b.id));
    let mut indirect = [a, b, c];
    indirect[0].parent_id = Some(indirect[2].id);
    assert!(matches!(
        validate_task_hierarchy(&indirect),
        Err(MinervaError::HierarchyCycle { .. })
    ));
}

#[test]
fn hierarchy_accepts_deep_trees_and_reports_missing_parents() {
    let ids = TaskIdAllocator::new(0);
    let a = task(ids.next_id(), None);
    let b = task(ids.next_id(), Some(a.id));
    let c = task(ids.next_id(), Some(b.id));
    let d = task(ids.next_id(), Some(c.id));
    assert!(validate_task_hierarchy(&[a.clone(), b.clone(), c.clone(), d]).is_ok());
    let missing = task(ids.next_id(), Some("TSK-999999".parse().unwrap()));
    assert!(matches!(
        validate_task_hierarchy(&[a, b, c, missing]),
        Err(MinervaError::TaskNotFound { .. })
    ));
}

fn task(id: minerva_domain::TaskId, parent_id: Option<minerva_domain::TaskId>) -> Task {
    Task {
        schema_version: 1,
        id,
        title: "Task".into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("todo").unwrap(),
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
        archive_state: ArchiveState::Active,
    }
}
