use minerva_domain::{Relationship, RelationshipId, RelationshipType, TaskIdAllocator};
use std::time::UNIX_EPOCH;

#[test]
fn relationship_accepts_valid_metadata() {
    let ids = TaskIdAllocator::new(0);
    let relationship = Relationship::new(Relationship {
        schema_version: 1,
        id: RelationshipId::new(),
        source_task: ids.next_id(),
        target_task: ids.next_id(),
        relationship_type: RelationshipType::DependsOn,
        reason: Some("Task B must land first".into()),
        created_at: UNIX_EPOCH,
    })
    .unwrap();
    assert_eq!(relationship.relationship_type, RelationshipType::DependsOn);
}
