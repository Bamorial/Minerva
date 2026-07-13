mod support;

use minerva_domain::MinervaError;
use minerva_storage::{MinervaLayout, read_task_declaration, write_task_declaration};
use std::fs;
use support::{sample_task, temp_repo};

#[test]
fn invalid_declarations_are_rejected_on_read_and_write() {
    let root = temp_repo("task-declaration");
    let layout = MinervaLayout::new(&root);
    let task = sample_task();
    fs::create_dir_all(layout.task_dir(task.id)).unwrap();
    let invalid = "# Declaration\n\n## Objective\n";
    let write_error = write_task_declaration(&layout, task.id, invalid).unwrap_err();
    assert!(matches!(
        write_error,
        MinervaError::SchemaError { reason, .. }
            if reason.contains("## Current State")
    ));
    fs::write(layout.declaration_file(task.id), invalid).unwrap();
    let read_error = read_task_declaration(&layout, task.id).unwrap_err();
    assert!(matches!(
        read_error,
        MinervaError::SchemaError { reason, .. }
            if reason.contains("## Current State")
    ));
}
