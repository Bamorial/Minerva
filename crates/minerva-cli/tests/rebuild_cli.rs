mod support;

use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn rebuild_command_recreates_missing_index_and_is_idempotent() {
    let root = temp_dir("cli-rebuild");
    assert!(run(&root, &["init"]).status.success());
    create_task(&root, task(1, "Rebuild the task index"));
    let index = root.join(".minerva/indexes/tasks.json");
    fs::remove_file(&index).unwrap();
    let rebuilt = run(&root, &["rebuild"]);
    assert!(rebuilt.status.success(), "{rebuilt:?}");
    assert!(
        str::from_utf8(&rebuilt.stdout)
            .unwrap()
            .contains("wrote .minerva/indexes/tasks.json")
    );
    let initial = fs::read_to_string(&index).unwrap();
    let repeated = run(&root, &["rebuild"]);
    assert!(repeated.status.success(), "{repeated:?}");
    assert!(
        str::from_utf8(&repeated.stdout)
            .unwrap()
            .contains("kept .minerva/indexes/tasks.json")
    );
    assert_eq!(fs::read_to_string(&index).unwrap(), initial);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn rebuild_dry_run_reports_changes_without_writing_index() {
    let root = temp_dir("cli-rebuild-dry-run");
    assert!(run(&root, &["init"]).status.success());
    create_task(&root, task(1, "Preview rebuild"));
    let index = root.join(".minerva/indexes/tasks.json");
    fs::remove_file(&index).unwrap();
    let output = run(&root, &["rebuild", "--dry-run"]);
    assert!(output.status.success(), "{output:?}");
    assert!(!index.exists());
    assert!(
        str::from_utf8(&output.stdout)
            .unwrap()
            .contains("would write .minerva/indexes/tasks.json")
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn rebuild_reports_invalid_tasks_and_rebuilds_from_valid_ones() {
    let root = temp_dir("cli-rebuild-invalid");
    assert!(run(&root, &["init"]).status.success());
    let valid = task(1, "Valid rebuild task");
    let invalid = task(2, "Broken rebuild task");
    create_task(&root, valid.clone());
    create_task(&root, invalid.clone());
    fs::write(
        root.join(".minerva/tasks").join(invalid.id.to_string()).join("task.yaml"),
        "schema_version: 99\nid: TSK-000002\ntitle: Broken rebuild task\n",
    )
    .unwrap();
    let index = root.join(".minerva/indexes/tasks.json");
    fs::remove_file(&index).unwrap();
    let output = run(&root, &["rebuild"]);
    assert!(!output.status.success(), "{output:?}");
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("wrote .minerva/indexes/tasks.json"));
    assert!(stderr.contains(&format!("invalid task {}", invalid.id)));
    let contents = fs::read_to_string(&index).unwrap();
    assert!(contents.contains(&valid.title));
    assert!(!contents.contains(&invalid.title));
    fs::remove_dir_all(root).unwrap();
}
