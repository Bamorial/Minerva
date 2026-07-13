mod support;

use minerva_application::{TaskCreateRecord, TaskRepository, TaskWriteResult};
use minerva_domain::{ArchiveState, MinervaError};
use minerva_storage::{FilesystemTaskRepository, MinervaLayout};
use std::fs;
use support::{task, temp_repo};

#[test]
fn repository_creates_updates_and_archives_tasks() {
    let root = temp_repo("task-repository-writes");
    let repo = FilesystemTaskRepository;
    let created = task(1, "Implement task repo");
    let record = TaskCreateRecord {
        task: created.clone(),
        instructions: "# Feature\n".into(),
        declaration: "# Declaration\n".into(),
    };
    let result = repo.create_task(&root, &record).unwrap();
    assert_eq!(result.previous_version, None);
    assert!(result.event_id.is_some());
    let task_dir = MinervaLayout::new(&root).task_dir(created.id);
    assert_eq!(
        fs::read_to_string(task_dir.join("instructions.md")).unwrap(),
        "# Feature\n"
    );
    assert_eq!(
        fs::read_to_string(task_dir.join("declaration.md")).unwrap(),
        "# Declaration\n"
    );
    assert!(
        fs::read_to_string(task_dir.join("events.jsonl"))
            .unwrap()
            .contains("task-created")
    );
    let mut updated = created.clone();
    updated.title = "Implement shared task repo".into();
    updated.version = created.version.next();
    assert_eq!(
        repo.update_task(&root, &updated).unwrap(),
        TaskWriteResult {
            previous_version: Some(created.version),
            current_version: updated.version,
            event_id: None,
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
    repo.create_task(
        &root,
        &TaskCreateRecord {
            task: task.clone(),
            instructions: "# Feature\n".into(),
            declaration: "# Declaration\n".into(),
        },
    )
    .unwrap();
    let mut updated = task.clone();
    updated.title = "Implement shared task repo".into();
    updated.version = task.version.next();
    repo.update_task(&root, &updated).unwrap();
    let error = repo.update_task(&root, &updated).unwrap_err();
    assert!(
        matches!(error, MinervaError::VersionConflict { expected, actual, .. } if expected == "3" && actual == "2")
    );
}
