use minerva_domain::{
    EventId, IdentifierError, ProjectId, RelationshipId, TaskId, TaskIdAllocator,
    TaskIdentity,
};
use std::num::NonZeroU32;
use std::str::FromStr;

#[test]
fn task_ids_use_stable_readable_format() {
    let id = TaskId::from_sequence(NonZeroU32::new(42).unwrap());
    assert_eq!(id.to_string(), "TSK-000042");
    assert_eq!(TaskId::from_str("TSK-000042").unwrap(), id);
}

#[test]
fn task_ids_reject_invalid_values() {
    assert!(matches!(
        TaskId::from_str("tsk-000001"),
        Err(IdentifierError::InvalidPrefix { .. })
    ));
    assert!(matches!(
        TaskId::from_str("TSK-000000"),
        Err(IdentifierError::InvalidBody { .. })
    ));
}

#[test]
fn task_id_allocator_emits_unique_ids() {
    let allocator = TaskIdAllocator::new(7);
    assert_eq!(allocator.next_id().to_string(), "TSK-000008");
    assert_eq!(allocator.next_id().to_string(), "TSK-000009");
}

#[test]
fn typed_ids_serialize_and_round_trip() {
    let project = ProjectId::new();
    let relationship = RelationshipId::new();
    let event = EventId::new();
    assert_eq!(
        serde_json::from_str::<ProjectId>(&serde_json::to_string(&project).unwrap())
            .unwrap(),
        project
    );
    assert_eq!(
        serde_json::from_str::<RelationshipId>(
            &serde_json::to_string(&relationship).unwrap()
        )
        .unwrap(),
        relationship
    );
    assert_eq!(
        serde_json::from_str::<EventId>(&serde_json::to_string(&event).unwrap())
            .unwrap(),
        event
    );
}

#[test]
fn ulid_backed_ids_reject_wrong_prefixes() {
    assert!(matches!(
        ProjectId::from_str("REL-01ARZ3NDEKTSV4RRFFQ69G5FAV"),
        Err(IdentifierError::InvalidPrefix { .. })
    ));
}

#[test]
fn task_identity_depends_on_id_not_title_or_folder() {
    let id = TaskId::from_sequence(NonZeroU32::new(1).unwrap());
    let left = TaskIdentity::new(id, "old title", "old-folder");
    let right = TaskIdentity::new(id, "new title", "new-folder");
    assert_eq!(left, right);
}
