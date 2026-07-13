mod support;

use minerva_application::{ProjectRepository, TaskRepository};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::fs;
use support::{create_record, task, temp_repo};

#[test]
fn validation_reports_missing_parents() {
    let root = temp_repo("validation-missing-parent");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let item = task(1, "Validate hierarchy");
    FilesystemTaskRepository.create_task(&root, &create_record(item.clone())).unwrap();
    let path = root.join(".minerva/tasks").join(item.id.to_string()).join("task.yaml");
    fs::write(
        &path,
        fs::read_to_string(&path)
            .unwrap()
            .replace("parent_id: null", "parent_id: TSK-000099"),
    )
    .unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "missing_parent", &item.id.to_string()));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validation_reports_hierarchy_cycles() {
    let root = temp_repo("validation-hierarchy-cycle");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let parent = task(1, "Parent");
    let mut child = task(2, "Child");
    child.parent_id = Some(parent.id);
    FilesystemTaskRepository
        .create_task(&root, &create_record(parent.clone()))
        .unwrap();
    FilesystemTaskRepository.create_task(&root, &create_record(child.clone())).unwrap();
    let path =
        root.join(".minerva/tasks").join(parent.id.to_string()).join("task.yaml");
    fs::write(
        &path,
        fs::read_to_string(&path)
            .unwrap()
            .replace("parent_id: null", &format!("parent_id: {}", child.id)),
    )
    .unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "hierarchy_cycle", &parent.id.to_string()));
    assert!(has(&result, "hierarchy_cycle", &child.id.to_string()));
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
