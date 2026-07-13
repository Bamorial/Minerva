mod support;

use minerva_application::TaskRepository;
use minerva_storage::{
    FilesystemTaskRepository, MinervaLayout, TaskIndexStatus, refresh_task_index,
    task_index_status, write_task,
};
use std::fs;
use support::{create_record, task, temp_repo};

#[test]
fn task_index_rebuild_is_deterministic() {
    let root = temp_repo("task-index-rebuild");
    let repo = FilesystemTaskRepository;
    repo.create_task(&root, &create_record(task(2, "Review task repo"))).unwrap();
    repo.create_task(&root, &create_record(task(1, "Implement task repo"))).unwrap();
    let path = MinervaLayout::new(&root).task_index_file();
    let initial = fs::read_to_string(&path).unwrap();
    fs::remove_file(&path).unwrap();
    refresh_task_index(&MinervaLayout::new(&root)).unwrap();
    assert_eq!(fs::read_to_string(path).unwrap(), initial);
}

#[test]
fn stale_or_missing_indexes_fall_back_safely() {
    let root = temp_repo("task-index-fallback");
    let repo = FilesystemTaskRepository;
    let mut first = task(1, "Implement task repo");
    repo.create_task(&root, &create_record(first.clone())).unwrap();
    let path = MinervaLayout::new(&root).task_index_file();
    fs::remove_file(&path).unwrap();
    assert_eq!(repo.resolve_task(&root, "Implement task repo").unwrap(), first);
    refresh_task_index(&MinervaLayout::new(&root)).unwrap();
    first.title = "Implement task index".into();
    first.version = first.version.next();
    write_task(&MinervaLayout::new(&root), &first).unwrap();
    assert_eq!(
        task_index_status(&MinervaLayout::new(&root)).unwrap(),
        TaskIndexStatus::Stale
    );
    assert_eq!(repo.resolve_task(&root, "Implement task index").unwrap(), first);
}
