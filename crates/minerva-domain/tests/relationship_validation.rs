use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, MinervaError, Relationship,
    RelationshipId, RelationshipType, StatusKey, Task, TaskIdAllocator, TaskPriority,
    TaskTypeKey, TaskVersion, validate_relationships,
};
use std::{collections::BTreeSet, time::UNIX_EPOCH};

#[test]
fn relationship_validation_rejects_self_links_and_blank_reasons() {
    let task_id = TaskIdAllocator::new(0).next_id();
    let self_link =
        Relationship::new(relationship(task_id, task_id, RelationshipType::Blocks));
    let blank_reason = Relationship::new(Relationship {
        reason: Some(" ".into()),
        ..relationship(
            task_id,
            TaskIdAllocator::new(1).next_id(),
            RelationshipType::References,
        )
    });
    assert!(
        matches!(self_link, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "target_task")
    );
    assert!(
        matches!(blank_reason, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "reason")
    );
}

#[test]
fn relationship_graph_rejects_duplicates_and_parent_records() {
    let ids = TaskIdAllocator::new(0);
    let left = task(ids.next_id(), None);
    let right = task(ids.next_id(), Some(left.id));
    let duplicate = relationship(left.id, right.id, RelationshipType::DependsOn);
    let parent = relationship(left.id, right.id, RelationshipType::Parent);
    assert!(
        matches!(validate_relationships(&[left.clone(), right.clone()], &[duplicate.clone(), duplicate]), Err(MinervaError::InvalidConfiguration { key, .. }) if key == "relationships")
    );
    assert!(
        matches!(validate_relationships(&[left, right], &[parent]), Err(MinervaError::InvalidConfiguration { key, .. }) if key == "relationship_type")
    );
}

#[test]
fn relationship_graph_rejects_reverse_semantic_duplicates() {
    let ids = TaskIdAllocator::new(0);
    let left = task(ids.next_id(), None);
    let right = task(ids.next_id(), None);
    let related = relationship(left.id, right.id, RelationshipType::RelatedTo);
    let reverse_related = relationship(right.id, left.id, RelationshipType::RelatedTo);
    let blocks = relationship(left.id, right.id, RelationshipType::Blocks);
    let depends_on = relationship(right.id, left.id, RelationshipType::DependsOn);
    assert!(
        matches!(validate_relationships(&[left.clone(), right.clone()], &[related, reverse_related]), Err(MinervaError::InvalidConfiguration { key, .. }) if key == "relationships")
    );
    assert!(
        matches!(validate_relationships(&[left, right], &[blocks, depends_on]), Err(MinervaError::InvalidConfiguration { key, .. }) if key == "relationships")
    );
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
        archive_state: ArchiveState::Active,
    }
}
