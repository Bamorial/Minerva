mod support;

use minerva_application::{ProjectRepository, TaskRepository};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::fs;
use support::{create_record, task, temp_repo};

#[test]
fn validation_reports_unknown_task_types_and_statuses() {
    let root = temp_repo("validation-task-model");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let item = task(1, "Validate model");
    FilesystemTaskRepository.create_task(&root, &create_record(item.clone())).unwrap();
    let path = root.join(".minerva/tasks").join(item.id.to_string()).join("task.yaml");
    let contents = fs::read_to_string(&path).unwrap();
    fs::write(&path, contents.replace("task_type: feature", "task_type: ghost"))
        .unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "invalid_task_type", &item.id.to_string()));
    fs::write(
        &path,
        fs::read_to_string(&path)
            .unwrap()
            .replace("status: in-progress", "status: ghost"),
    )
    .unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "illegal_status", &item.id.to_string()));
    fs::remove_dir_all(root).unwrap();
}

fn has(
    result: &minerva_application::ProjectValidationResult,
    code: &str,
    task_ref: &str,
) -> bool {
    result
        .findings
        .iter()
        .any(|item| item.code == code && item.task_ref.as_deref() == Some(task_ref))
}
