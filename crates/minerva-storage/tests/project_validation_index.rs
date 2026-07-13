mod support;

use minerva_application::{ProjectRepository, TaskRepository};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::{fs, thread, time::Duration};
use support::{create_record, task, temp_repo};

#[test]
fn validation_reports_index_status() {
    let root = temp_repo("validation-index");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let initial = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&initial, "stale_index", "warning"));
    let item = task(1, "Index drift");
    FilesystemTaskRepository.create_task(&root, &create_record(item)).unwrap();
    let fresh = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&fresh, "stale_index", "information"));
    thread::sleep(Duration::from_millis(20));
    fs::remove_file(root.join(".minerva/indexes/tasks.json")).unwrap();
    let missing = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&missing, "stale_index", "warning"));
    fs::remove_dir_all(root).unwrap();
}

fn has(
    result: &minerva_application::ProjectValidationResult,
    code: &str,
    severity: &str,
) -> bool {
    result.findings.iter().any(|item| {
        item.code == code
            && format!("{:?}", item.severity).eq_ignore_ascii_case(severity)
    })
}
