mod support;

use minerva_application::{TaskCreateRecord, TaskRepository};
use minerva_domain::{MinervaError, RelationshipType};
use minerva_storage::FilesystemTaskRepository;
use support::{relationship, task, temp_repo};

#[test]
fn repository_manages_relationships_and_queries_by_source_and_target() {
    let root = temp_repo("task-repository-relationships");
    let repo = FilesystemTaskRepository;
    let left = task(1, "Plan relationship storage");
    let right = task(2, "Implement relationship storage");
    let third = task(3, "Document relationship storage");
    for task in [&left, &right, &third] {
        repo.create_task(
            &root,
            &TaskCreateRecord {
                task: task.clone(),
                instructions: "# Feature\n".into(),
                declaration: "# Declaration\n".into(),
            },
        )
        .unwrap();
    }
    let kinds = [
        (RelationshipType::DependsOn, left.id, right.id),
        (RelationshipType::Blocks, right.id, third.id),
        (RelationshipType::RelatedTo, left.id, third.id),
        (RelationshipType::Duplicates, right.id, left.id),
        (RelationshipType::Implements, third.id, left.id),
        (RelationshipType::References, third.id, right.id),
    ];
    let mut created = Vec::new();
    for (index, (kind, source, target)) in kinds.into_iter().enumerate() {
        let reason = (index == 0).then_some("Must land first");
        created.push(
            repo.create_relationship(
                &root,
                &relationship(source, target, kind, reason),
            )
            .unwrap(),
        );
    }
    assert_eq!(repo.list_relationships(&root).unwrap().len(), created.len());
    assert_eq!(repo.list_relationships_from(&root, left.id).unwrap().len(), 2);
    assert_eq!(repo.list_relationships_to(&root, right.id).unwrap().len(), 2);
    assert_eq!(
        repo.list_relationships_from(&root, left.id).unwrap()[0].reason.as_deref(),
        Some("Must land first")
    );
    let removed = repo.remove_relationship(&root, created[1].id).unwrap();
    assert_eq!(removed.relationship_type, RelationshipType::Blocks);
    assert_eq!(repo.list_relationships(&root).unwrap().len(), created.len() - 1);
}

#[test]
fn repository_rejects_reverse_semantic_duplicates_and_missing_removals() {
    let root = temp_repo("task-repository-relationship-validation");
    let repo = FilesystemTaskRepository;
    let left = task(1, "Plan relationship storage");
    let right = task(2, "Implement relationship storage");
    for task in [&left, &right] {
        repo.create_task(&root, &support::create_record(task.clone())).unwrap();
    }
    repo.create_relationship(
        &root,
        &relationship(left.id, right.id, RelationshipType::Blocks, None),
    )
    .unwrap();
    let duplicate = repo
        .create_relationship(
            &root,
            &relationship(right.id, left.id, RelationshipType::DependsOn, None),
        )
        .unwrap_err();
    assert!(matches!(
        duplicate,
        MinervaError::InvalidConfiguration { key, .. } if key == "relationships"
    ));
    let missing = repo
        .remove_relationship(
            &root,
            relationship(left.id, right.id, RelationshipType::References, None).id,
        )
        .unwrap_err();
    assert!(matches!(
        missing,
        MinervaError::InvalidConfiguration { key, .. } if key == "relationship_id"
    ));
}
