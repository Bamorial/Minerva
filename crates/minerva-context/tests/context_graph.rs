use minerva_context::{
    ContextGraphSelector, ContextInclusionReason, ContextRelationshipDirection,
};
use minerva_domain::{
    ArchiveState, ContextDetail, ContextPolicy, ContextRelationPolicy,
    DeclarationActor, DeclarationMetadata, Relationship, RelationshipId,
    RelationshipType, StatusKey, Task, TaskFacts, TaskId, TaskIdAllocator,
    TaskPriority, TaskTypeKey, TaskVersion,
};
use std::collections::BTreeSet;
use std::time::UNIX_EPOCH;

#[test]
fn selector_traverses_mixed_graph_deterministically() {
    let tasks = mixed_graph_tasks();
    let target = tasks[2].id;
    let relationships = vec![
        relationship(target, tasks[3].id, RelationshipType::DependsOn),
        relationship(tasks[3].id, tasks[4].id, RelationshipType::DependsOn),
        relationship(target, tasks[5].id, RelationshipType::References),
        relationship(tasks[6].id, target, RelationshipType::Implements),
        relationship(tasks[5].id, tasks[7].id, RelationshipType::RelatedTo),
        relationship(target, tasks[10].id, RelationshipType::References),
        relationship(target, tasks[12].id, RelationshipType::DependsOn),
    ];
    let selector = ContextGraphSelector::new(&tasks, &relationships).unwrap();
    let policy = policy();
    let selection = selector.select(target, &policy).unwrap();
    let again = selector.select(target, &policy).unwrap();
    assert_eq!(selection, again);
    assert_eq!(
        selection.task_ids(),
        vec![
            target,
            tasks[0].id,
            tasks[1].id,
            tasks[3].id,
            tasks[4].id,
            tasks[5].id,
            tasks[6].id,
            tasks[10].id,
            tasks[7].id,
            tasks[8].id,
            tasks[9].id,
        ]
    );
    assert_eq!(selection.items[0].reason, ContextInclusionReason::Target);
    assert_eq!(
        selection.items[1].reason,
        ContextInclusionReason::Ancestor { depth: 2 }
    );
    assert_eq!(
        selection.items[3].reason,
        ContextInclusionReason::Dependency {
            depth: 1,
            relationship_type: RelationshipType::DependsOn,
        }
    );
    assert_eq!(
        selection.items[5].reason,
        ContextInclusionReason::RelatedTask {
            depth: 1,
            relationship_type: RelationshipType::References,
            direction: ContextRelationshipDirection::Outgoing,
        }
    );
    assert_eq!(
        selection.items[7].reason,
        ContextInclusionReason::RelatedTask {
            depth: 1,
            relationship_type: RelationshipType::References,
            direction: ContextRelationshipDirection::Outgoing,
        }
    );
    assert_eq!(selection.items[9].reason, ContextInclusionReason::Child { depth: 1 });
}

#[test]
fn selector_eliminates_duplicates_and_handles_cycles() {
    let tasks = vec![
        task(1, "Target"),
        task(2, "Dependency"),
        task(3, "Related"),
        task(4, "Loop"),
    ];
    let target = tasks[0].id;
    let relationships = vec![
        relationship(target, tasks[1].id, RelationshipType::DependsOn),
        relationship(tasks[1].id, target, RelationshipType::DependsOn),
        relationship(target, tasks[2].id, RelationshipType::References),
        relationship(tasks[2].id, tasks[3].id, RelationshipType::References),
        relationship(tasks[3].id, tasks[2].id, RelationshipType::References),
        relationship(target, tasks[1].id, RelationshipType::References),
    ];
    let policy = ContextPolicy {
        ancestors: None,
        dependencies: Some(scope(3)),
        related_tasks: Some(scope(3)),
        children: None,
        siblings: None,
        ..ContextPolicy::strict()
    };
    let selection = ContextGraphSelector::new(&tasks, &relationships)
        .unwrap()
        .select(target, &policy)
        .unwrap();
    assert_eq!(
        selection.task_ids(),
        vec![target, tasks[1].id, tasks[2].id, tasks[3].id]
    );
    assert_eq!(
        selection.items[1].reason,
        ContextInclusionReason::Dependency {
            depth: 1,
            relationship_type: RelationshipType::DependsOn,
        }
    );
}

fn mixed_graph_tasks() -> Vec<Task> {
    let mut tasks = vec![
        task(1, "Root"),
        task(2, "Parent"),
        task(3, "Target"),
        task(4, "Direct dependency"),
        task(5, "Transitive dependency"),
        task(6, "Related outgoing"),
        task(7, "Related incoming"),
        task(8, "Related transitive"),
        task(9, "Child"),
        task(10, "Grandchild"),
        task(11, "Sibling also related"),
        task(12, "Archived child"),
        completed_task(13, "Completed dependency"),
    ];
    tasks[1].parent_id = Some(tasks[0].id);
    tasks[2].parent_id = Some(tasks[1].id);
    tasks[8].parent_id = Some(tasks[2].id);
    tasks[9].parent_id = Some(tasks[8].id);
    tasks[10].parent_id = Some(tasks[1].id);
    tasks[11].parent_id = Some(tasks[2].id);
    tasks[11].archive_state = ArchiveState::Archived;
    tasks
}

fn policy() -> ContextPolicy {
    ContextPolicy {
        ancestors: Some(scope(2)),
        dependencies: Some(scope(2)),
        related_tasks: Some(scope(2)),
        children: Some(scope(2)),
        siblings: Some(scope(1)),
        include_archived: false,
        include_completed: false,
        ..ContextPolicy::strict()
    }
}

fn scope(depth: u8) -> ContextRelationPolicy {
    ContextRelationPolicy { detail: ContextDetail::Summary, depth }
}

fn completed_task(sequence: u32, title: &str) -> Task {
    let mut task = task(sequence, title);
    task.status = StatusKey::new("completed").unwrap();
    task.completed_at = Some(UNIX_EPOCH);
    task
}

fn task(sequence: u32, title: &str) -> Task {
    let allocator = TaskIdAllocator::new(sequence - 1);
    Task::new(Task {
        schema_version: 1,
        id: allocator.next_id(),
        title: title.into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("backlog").unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: BTreeSet::default(),
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
    })
    .unwrap()
}

fn relationship(
    source: TaskId,
    target: TaskId,
    relationship_type: RelationshipType,
) -> Relationship {
    Relationship::new(Relationship {
        schema_version: 1,
        id: RelationshipId::new(),
        source_task: source,
        target_task: target,
        relationship_type,
        reason: None,
        created_at: UNIX_EPOCH,
    })
    .unwrap()
}
