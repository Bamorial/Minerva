mod support;

use minerva_application::{ProjectRepository, TaskRepository};
use minerva_domain::RelationshipType;
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::fs;
use support::{create_record, relationship, task, temp_repo};

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
    assert!(has(&result, "missing_parent"));
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
    assert!(has(&result, "hierarchy_cycle"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validation_reports_dependency_cycles() {
    let root = temp_repo("validation-dependency-cycle");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let a = task(1, "A");
    let b = task(2, "B");
    FilesystemTaskRepository.create_task(&root, &create_record(a.clone())).unwrap();
    FilesystemTaskRepository.create_task(&root, &create_record(b.clone())).unwrap();
    FilesystemTaskRepository
        .create_relationship(
            &root,
            &relationship(a.id, b.id, RelationshipType::DependsOn, Some("need")),
        )
        .unwrap();
    let path =
        root.join(".minerva/tasks").join(a.id.to_string()).join("relationships.yaml");
    let reverse = swap_ids(
        fs::read_to_string(&path).unwrap(),
        a.id.to_string(),
        b.id.to_string(),
    );
    fs::write(
        root.join(".minerva/tasks").join(b.id.to_string()).join("relationships.yaml"),
        reverse,
    )
    .unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "invalid_dependency"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validation_reports_duplicate_relationships() {
    let root = temp_repo("validation-duplicate-relationship");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let a = task(1, "A");
    let b = task(2, "B");
    FilesystemTaskRepository.create_task(&root, &create_record(a.clone())).unwrap();
    FilesystemTaskRepository.create_task(&root, &create_record(b.clone())).unwrap();
    FilesystemTaskRepository
        .create_relationship(
            &root,
            &relationship(a.id, b.id, RelationshipType::RelatedTo, Some("match")),
        )
        .unwrap();
    let path =
        root.join(".minerva/tasks").join(a.id.to_string()).join("relationships.yaml");
    let current = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{current}\n{current}")).unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "duplicate_relationship"));
    fs::remove_dir_all(root).unwrap();
}

fn swap_ids(contents: String, first: String, second: String) -> String {
    contents
        .replace(&first, "__FIRST__")
        .replace(&second, "__SECOND__")
        .replace("__FIRST__", &second)
        .replace("__SECOND__", &first)
}

fn has(result: &minerva_application::ProjectValidationResult, code: &str) -> bool {
    result.findings.iter().any(|item| item.code == code)
}
