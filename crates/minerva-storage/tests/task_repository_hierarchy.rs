mod support;

use minerva_application::TaskRepository;
use minerva_storage::FilesystemTaskRepository;
use std::fs;
use support::{create_record, task, temp_repo};

#[test]
fn repository_moves_tasks_between_valid_parents() {
    let root = temp_repo("task-repository-reparent");
    let repo = FilesystemTaskRepository;
    let left = task(1, "Left parent");
    let right = task(2, "Right parent");
    let mut child = task(3, "Child");
    child.parent_id = Some(left.id);
    for item in [&left, &right, &child] {
        repo.create_task(&root, &create_record(item.clone())).unwrap();
    }
    let moved = repo.move_task(&root, child.id, Some(right.id), child.version).unwrap();
    assert_eq!(repo.read_task(&root, child.id).unwrap().parent_id, Some(right.id));
    assert!(moved.1.event_id.is_some());
    let events = fs::read_to_string(
        root.join(".minerva/tasks").join(child.id.to_string()).join("events.jsonl"),
    )
    .unwrap();
    assert!(events.contains("\"kind\":\"task-moved\""));
    assert!(events.contains(&format!("\"from_parent_id\":\"{}\"", left.id)));
    assert!(events.contains(&format!("\"to_parent_id\":\"{}\"", right.id)));
}

#[test]
fn repository_moves_tasks_to_root() {
    let root = temp_repo("task-repository-unparent");
    let repo = FilesystemTaskRepository;
    let parent = task(1, "Parent");
    let mut child = task(2, "Child");
    child.parent_id = Some(parent.id);
    repo.create_task(&root, &create_record(parent)).unwrap();
    repo.create_task(&root, &create_record(child.clone())).unwrap();
    let moved = repo.move_task(&root, child.id, None, child.version).unwrap();
    assert_eq!(repo.read_task(&root, child.id).unwrap().parent_id, None);
    assert!(moved.1.event_id.is_some());
}

#[test]
fn repository_rejects_moves_that_create_cycles() {
    let root = temp_repo("task-repository-cycle");
    let repo = FilesystemTaskRepository;
    let root_task = task(1, "Root");
    let mut child = task(2, "Child");
    let mut grandchild = task(3, "Grandchild");
    child.parent_id = Some(root_task.id);
    grandchild.parent_id = Some(child.id);
    for item in [&root_task, &child, &grandchild] {
        repo.create_task(&root, &create_record(item.clone())).unwrap();
    }
    let error =
        repo.move_task(&root, root_task.id, Some(grandchild.id), root_task.version);
    assert!(matches!(error, Err(minerva_domain::MinervaError::HierarchyCycle { .. })));
}
