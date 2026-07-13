mod support;

use minerva_application::TaskRepository;
use minerva_domain::MinervaError;
use minerva_storage::{FilesystemTaskRepository, MinervaLayout, TaskLock};
use support::{create_record, task, temp_repo};

#[test]
fn write_operations_use_per_task_locks() {
    let root = temp_repo("task-repository-locks");
    let repo = FilesystemTaskRepository;
    let created = task(1, "Implement task repo");
    repo.create_task(&root, &create_record(created.clone())).unwrap();
    let layout = MinervaLayout::new(&root);
    let _lock = TaskLock::acquire(&layout, created.id).unwrap();
    let mut updated = created.clone();
    updated.version = created.version.next();
    let error = repo.update_task(&root, &updated).unwrap_err();
    assert_eq!(
        error,
        MinervaError::LockConflict {
            path: layout.task_lock_file(created.id).display().to_string(),
        }
    );
}
