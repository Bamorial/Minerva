use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, MinervaError, Relationship,
    RelationshipId, RelationshipType, StatusKey, Task, TaskIdAllocator, TaskPriority,
    TaskTypeKey, TaskVersion, validate_relationships,
};
use std::{collections::BTreeSet, time::UNIX_EPOCH};

#[test]
fn dependencies_reject_self_links_and_missing_targets() {
    let ids = TaskIdAllocator::new(0);
    let task = task(ids.next_id());
    let self_link =
        Relationship::new(relationship(task.id, task.id, RelationshipType::DependsOn));
    let missing = validate_relationships(
        &[task.clone()],
        &[relationship(task.id, ids.next_id(), RelationshipType::DependsOn)],
    );
    assert!(
        matches!(self_link, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "target_task")
    );
    assert!(
        matches!(missing, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "target_task")
    );
}

#[test]
fn dependencies_reject_direct_and_indirect_cycles() {
    let ids = TaskIdAllocator::new(0);
    let a = task(ids.next_id());
    let b = task(ids.next_id());
    let c = task(ids.next_id());
    let direct = [
        relationship(a.id, b.id, RelationshipType::DependsOn),
        relationship(b.id, a.id, RelationshipType::DependsOn),
    ];
    let indirect = [
        relationship(a.id, b.id, RelationshipType::DependsOn),
        relationship(b.id, c.id, RelationshipType::DependsOn),
        relationship(c.id, a.id, RelationshipType::DependsOn),
    ];
    assert!(matches!(
        validate_relationships(&[a.clone(), b.clone()], &direct),
        Err(MinervaError::DependencyCycle { .. })
    ));
    assert!(matches!(
        validate_relationships(&[a, b, c], &indirect),
        Err(MinervaError::DependencyCycle { .. })
    ));
}

#[test]
fn blocks_relationships_participate_in_dependency_cycles() {
    let ids = TaskIdAllocator::new(0);
    let a = task(ids.next_id());
    let b = task(ids.next_id());
    let cycle = [
        relationship(a.id, b.id, RelationshipType::Blocks),
        relationship(b.id, a.id, RelationshipType::Blocks),
    ];
    assert!(matches!(
        validate_relationships(&[a, b], &cycle),
        Err(MinervaError::DependencyCycle { .. })
    ));
}

fn relationship(
    source: minerva_domain::TaskId,
    target: minerva_domain::TaskId,
    kind: RelationshipType,
) -> Relationship {
    Relationship {
        schema_version: 1,
        id: RelationshipId::new(),
        source_task: source,
        target_task: target,
        relationship_type: kind,
        reason: None,
        created_at: UNIX_EPOCH,
    }
}

fn task(id: minerva_domain::TaskId) -> Task {
    Task {
        schema_version: 1,
        id,
        title: "Task".into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("todo").unwrap(),
        parent_id: None,
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
        archive_state: ArchiveState::Active,
    }
}
