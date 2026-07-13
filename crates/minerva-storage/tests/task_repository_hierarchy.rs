mod support;

use minerva_application::TaskRepository;
use minerva_storage::FilesystemTaskRepository;
use support::{create_record, task, temp_repo};

#[test]
fn repository_allows_reparenting_between_valid_parents() {
    let root = temp_repo("task-repository-reparent");
    let repo = FilesystemTaskRepository;
    let left = task(1, "Left parent");
    let right = task(2, "Right parent");
    let mut child = task(3, "Child");
    child.parent_id = Some(left.id);
    for item in [&left, &right, &child] {
        repo.create_task(&root, &create_record(item.clone())).unwrap();
    }
    child.parent_id = Some(right.id);
    child.version = child.version.next();
    repo.update_task(&root, &child).unwrap();
    assert_eq!(repo.read_task(&root, child.id).unwrap().parent_id, Some(right.id));
}

#[test]
fn repository_allows_removing_a_parent() {
    let root = temp_repo("task-repository-unparent");
    let repo = FilesystemTaskRepository;
    let parent = task(1, "Parent");
    let mut child = task(2, "Child");
    child.parent_id = Some(parent.id);
    repo.create_task(&root, &create_record(parent)).unwrap();
    repo.create_task(&root, &create_record(child.clone())).unwrap();
    child.parent_id = None;
    child.version = child.version.next();
    repo.update_task(&root, &child).unwrap();
    assert_eq!(repo.read_task(&root, child.id).unwrap().parent_id, None);
}
