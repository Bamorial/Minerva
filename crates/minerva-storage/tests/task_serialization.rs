mod support;

use minerva_domain::MinervaError;
use minerva_storage::{
    MinervaLayout, read_task, read_task_declaration, read_task_instructions,
    read_task_notes, write_task, write_task_declaration, write_task_instructions,
    write_task_notes,
};
use std::fs;
use support::{sample_task, temp_repo};

#[test]
fn task_storage_round_trips_yaml_and_markdown() {
    let root = temp_repo("task-round-trip");
    let layout = MinervaLayout::new(&root);
    let task = sample_task();
    write_task(&layout, &task).unwrap();
    write_task_instructions(&layout, task.id, "# Instructions\n").unwrap();
    write_task_declaration(&layout, task.id, "# Declaration\n").unwrap();
    write_task_notes(&layout, task.id, "# Notes\n").unwrap();
    assert_eq!(read_task(&layout, task.id).unwrap(), task);
    assert_eq!(read_task_instructions(&layout, task.id).unwrap(), "# Instructions\n");
    assert_eq!(read_task_declaration(&layout, task.id).unwrap(), "# Declaration\n");
    assert_eq!(read_task_notes(&layout, task.id).unwrap(), "# Notes\n");
}

#[test]
fn optional_markdown_defaults_and_required_declaration_errors() {
    let root = temp_repo("task-defaults");
    let layout = MinervaLayout::new(&root);
    let task = sample_task();
    fs::create_dir_all(layout.task_dir(task.id)).unwrap();
    assert_eq!(read_task_instructions(&layout, task.id).unwrap(), "");
    assert_eq!(read_task_notes(&layout, task.id).unwrap(), "");
    let error = read_task_declaration(&layout, task.id).unwrap_err();
    assert!(
        matches!(error, MinervaError::SchemaError { reason, .. } if reason == "required file is missing")
    );
}

#[test]
fn missing_and_malformed_task_yaml_are_rejected() {
    let root = temp_repo("task-malformed");
    let layout = MinervaLayout::new(&root);
    let task = sample_task();
    let missing = read_task(&layout, task.id).unwrap_err();
    assert!(
        matches!(missing, MinervaError::SchemaError { reason, .. } if reason == "required file is missing")
    );
    fs::create_dir_all(layout.task_dir(task.id)).unwrap();
    fs::write(layout.task_file(task.id), "schema_version: [").unwrap();
    assert!(matches!(
        read_task(&layout, task.id).unwrap_err(),
        MinervaError::SchemaError { .. }
    ));
}

#[test]
fn unsupported_task_schema_versions_are_rejected() {
    let root = temp_repo("task-schema");
    let layout = MinervaLayout::new(&root);
    let task = sample_task();
    write_task(&layout, &task).unwrap();
    let yaml = fs::read_to_string(layout.task_file(task.id)).unwrap();
    fs::write(
        layout.task_file(task.id),
        yaml.replace("schema_version: 1", "schema_version: 2"),
    )
    .unwrap();
    let error = read_task(&layout, task.id).unwrap_err();
    assert!(
        matches!(error, MinervaError::SchemaError { reason, .. } if reason.contains("unsupported schema version `2`"))
    );
}

#[test]
fn stale_task_versions_report_conflicts() {
    let root = temp_repo("task-conflict");
    let layout = MinervaLayout::new(&root);
    let task = sample_task();
    write_task(&layout, &task).unwrap();
    let error = write_task(&layout, &task).unwrap_err();
    assert!(
        matches!(error, MinervaError::VersionConflict { expected, actual, .. } if expected == "2" && actual == "1")
    );
}
