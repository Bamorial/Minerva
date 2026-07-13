mod support;

use minerva_application::{ProjectRepository, TaskRepository};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::fs;
use support::{create_record, task, temp_repo};

#[test]
fn validation_reports_missing_task_files_and_bad_declarations() {
    let root = temp_repo("validation-task-files");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let item = task(1, "Validate files");
    FilesystemTaskRepository.create_task(&root, &create_record(item.clone())).unwrap();
    let dir = root.join(".minerva/tasks").join(item.id.to_string());
    fs::remove_file(dir.join("notes.md")).unwrap();
    fs::write(dir.join("declaration.md"), "# Declaration\n").unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "missing_file", "notes.md", &item.id.to_string()));
    assert!(has(&result, "invalid_declaration", "task.yaml", &item.id.to_string()));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validation_reports_malformed_event_logs() {
    let root = temp_repo("validation-event-log");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let item = task(1, "Validate events");
    FilesystemTaskRepository.create_task(&root, &create_record(item.clone())).unwrap();
    let path =
        root.join(".minerva/tasks").join(item.id.to_string()).join("events.jsonl");
    fs::write(path, "{not json}\n").unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "malformed_event_log", "events.jsonl", &item.id.to_string()));
    fs::remove_dir_all(root).unwrap();
}

fn has(
    result: &minerva_application::ProjectValidationResult,
    code: &str,
    suffix: &str,
    task_ref: &str,
) -> bool {
    result.findings.iter().any(|item| {
        item.code == code
            && item.path.ends_with(suffix)
            && item.task_ref.as_deref() == Some(task_ref)
    })
}
