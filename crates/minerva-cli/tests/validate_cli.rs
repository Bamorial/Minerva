mod support;

use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn validate_reports_clean_project_with_info_exit_zero() {
    let root = temp_dir("cli-validate-clean");
    assert!(run(&root, &["init"]).status.success());
    create_task(&root, task(1, "Validate clean project"));
    let output = run(&root, &["validate"]);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(str::from_utf8(&output.stdout).unwrap().contains("validate project:"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validate_reports_warning_exit_code_for_missing_index() {
    let root = temp_dir("cli-validate-warning");
    assert!(run(&root, &["init"]).status.success());
    let output = run(&root, &["validate"]);
    assert_eq!(output.status.code(), Some(23), "{output:?}");
    assert!(str::from_utf8(&output.stderr).unwrap().contains("stale_index"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validate_reports_error_exit_code_for_invalid_task() {
    let root = temp_dir("cli-validate-error");
    assert!(run(&root, &["init"]).status.success());
    let broken = task(1, "Broken task");
    create_task(&root, broken.clone());
    let path =
        root.join(".minerva/tasks").join(broken.id.to_string()).join("task.yaml");
    fs::write(
        &path,
        fs::read_to_string(&path)
            .unwrap()
            .replace("task_type: feature", "task_type: ghost"),
    )
    .unwrap();
    let output = run(&root, &["validate"]);
    assert_eq!(output.status.code(), Some(24), "{output:?}");
    assert!(str::from_utf8(&output.stderr).unwrap().contains("invalid_task_type"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validate_task_scope_filters_findings_to_the_requested_task() {
    let root = temp_dir("cli-validate-task");
    assert!(run(&root, &["init"]).status.success());
    let clean = task(1, "Clean task");
    let broken = task(2, "Broken task");
    create_task(&root, clean.clone());
    create_task(&root, broken.clone());
    let path =
        root.join(".minerva/tasks").join(broken.id.to_string()).join("task.yaml");
    fs::write(
        &path,
        fs::read_to_string(&path)
            .unwrap()
            .replace("task_type: feature", "task_type: ghost"),
    )
    .unwrap();
    let output = run(&root, &["validate", &clean.id.to_string()]);
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains(&clean.id.to_string()));
    assert!(stdout.contains("no findings"));
    let scoped = run(&root, &["validate", &broken.id.to_string()]);
    assert_eq!(scoped.status.code(), Some(24), "{scoped:?}");
    let stderr = str::from_utf8(&scoped.stderr).unwrap();
    assert!(stderr.contains(&broken.id.to_string()));
    assert!(!stderr.contains(&clean.id.to_string()));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn validate_json_reports_summary_and_exit_code() {
    let root = temp_dir("cli-validate-json");
    assert!(run(&root, &["init"]).status.success());
    let output = run(&root, &["--json", "validate"]);
    assert_eq!(output.status.code(), Some(23), "{output:?}");
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("\"code\":\"validation_warning\""));
    assert!(stderr.contains("\"exit_code\":23"));
    assert!(stderr.contains("\"summary\""));
    fs::remove_dir_all(root).unwrap();
}
