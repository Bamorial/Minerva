mod support;

use minerva_application::{TaskRepository, TaskWriteResult};
use minerva_domain::{ArchiveState, MinervaError};
use minerva_storage::FilesystemTaskRepository;
use support::{task, temp_repo};

#[test]
fn repository_creates_updates_and_archives_tasks() {
    let root = temp_repo("task-repository-writes");
    let repo = FilesystemTaskRepository;
    let created = task(1, "Implement task repo");
    assert_eq!(
        repo.create_task(&root, &created).unwrap(),
        TaskWriteResult { previous_version: None, current_version: created.version }
    );
    let mut updated = created.clone();
    updated.title = "Implement shared task repo".into();
    updated.version = created.version.next();
    assert_eq!(
        repo.update_task(&root, &updated).unwrap(),
        TaskWriteResult {
            previous_version: Some(created.version),
            current_version: updated.version
        }
    );
    let archived = repo.archive_task(&root, updated.id, updated.version).unwrap();
    assert_eq!(archived.previous_version, Some(updated.version));
    assert_eq!(archived.current_version, updated.version.next());
    assert_eq!(
        repo.read_task(&root, updated.id).unwrap().archive_state,
        ArchiveState::Archived
    );
}

#[test]
fn stale_task_updates_report_version_conflicts() {
    let root = temp_repo("task-repository-conflicts");
    let repo = FilesystemTaskRepository;
    let task = task(1, "Implement task repo");
    repo.create_task(&root, &task).unwrap();
    let mut updated = task.clone();
    updated.title = "Implement shared task repo".into();
    updated.version = task.version.next();
    repo.update_task(&root, &updated).unwrap();
    let error = repo.update_task(&root, &updated).unwrap_err();
    assert!(
        matches!(error, MinervaError::VersionConflict { expected, actual, .. } if expected == "3" && actual == "2")
    );
}
