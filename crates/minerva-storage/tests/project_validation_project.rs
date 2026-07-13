mod support;

use minerva_application::{ProjectRepository, TaskRepository};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::fs;
use support::temp_repo;

#[test]
fn validation_reports_project_schema_and_malformed_config() {
    let root = temp_repo("validation-project");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let path = root.join(".minerva/project.yaml");
    fs::write(
        &path,
        fs::read_to_string(&path)
            .unwrap()
            .replace("schema_version: 1", "schema_version: 9"),
    )
    .unwrap();
    fs::write(root.join(".minerva/config.yaml"), ":\n").unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "schema_version", ".minerva/project.yaml", None));
    assert!(has(&result, "malformed_yaml", ".minerva/config.yaml", None));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validation_reports_missing_project_files() {
    let root = temp_repo("validation-project-missing");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    fs::remove_file(root.join(".minerva/instructions.md")).unwrap();
    let result = FilesystemTaskRepository.validate_project_state(&root).unwrap();
    assert!(has(&result, "missing_file", ".minerva/instructions.md", None));
    fs::remove_dir_all(root).unwrap();
}

fn has(
    result: &minerva_application::ProjectValidationResult,
    code: &str,
    suffix: &str,
    task_ref: Option<&str>,
) -> bool {
    result.findings.iter().any(|item| {
        item.code == code
            && item.path.ends_with(suffix)
            && item.task_ref.as_deref() == task_ref
    })
}
