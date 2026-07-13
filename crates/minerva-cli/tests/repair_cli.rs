mod support;

use serde_json::Value;
use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn repair_command_recreates_safe_state_and_reports_each_change() {
    let root = temp_dir("cli-repair");
    assert!(run(&root, &["init"]).status.success());
    let item = task(1, "Repair safe state");
    create_task(&root, item.clone());
    fs::remove_dir_all(root.join(".minerva/contexts")).unwrap();
    fs::remove_dir_all(root.join(".minerva/sessions")).unwrap();
    fs::remove_dir_all(root.join(".minerva/locks")).unwrap();
    fs::remove_file(root.join(".minerva/indexes/tasks.json")).unwrap();
    fs::remove_file(
        root.join(".minerva/tasks").join(item.id.to_string()).join("notes.md"),
    )
    .unwrap();
    fs::write(root.join(".minerva/indexes/.tasks.json.tmp.123"), "temp").unwrap();
    let output = run(&root, &["--json", "repair"]);
    assert!(output.status.success(), "{output:?}");
    assert!(root.join(".minerva/contexts").is_dir());
    assert!(root.join(".minerva/sessions").is_dir());
    assert!(root.join(".minerva/locks").is_dir());
    assert!(root.join(".minerva/indexes/tasks.json").is_file());
    assert!(
        root.join(".minerva/tasks")
            .join(item.id.to_string())
            .join("notes.md")
            .is_file()
    );
    assert!(!root.join(".minerva/indexes/.tasks.json.tmp.123").exists());
    assert!(
        backup_files(&root).iter().any(|path| path.contains("safe-repair-backup-"))
    );
    let json = json(&output.stdout);
    assert_eq!(json["result"]["operations"].as_array().unwrap().len(), 6);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn repair_dry_run_reports_changes_without_writing() {
    let root = temp_dir("cli-repair-dry-run");
    assert!(run(&root, &["init"]).status.success());
    let item = task(1, "Preview repair");
    create_task(&root, item.clone());
    fs::remove_dir_all(root.join(".minerva/contexts")).unwrap();
    fs::remove_file(root.join(".minerva/indexes/tasks.json")).unwrap();
    fs::remove_file(
        root.join(".minerva/tasks").join(item.id.to_string()).join("notes.md"),
    )
    .unwrap();
    fs::write(root.join(".minerva/indexes/.tasks.json.tmp.456"), "temp").unwrap();
    let output = run(&root, &["repair", "--dry-run"]);
    assert!(output.status.success(), "{output:?}");
    assert!(!root.join(".minerva/contexts").exists());
    assert!(!root.join(".minerva/indexes/tasks.json").exists());
    assert!(
        !root
            .join(".minerva/tasks")
            .join(item.id.to_string())
            .join("notes.md")
            .exists()
    );
    assert!(root.join(".minerva/indexes/.tasks.json.tmp.456").exists());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("would create"));
    assert!(stdout.contains("would remove"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn repair_reports_invalid_tasks_as_issues() {
    let root = temp_dir("cli-repair-issues");
    assert!(run(&root, &["init"]).status.success());
    let valid = task(1, "Valid repair task");
    let invalid = task(2, "Broken repair task");
    create_task(&root, valid.clone());
    create_task(&root, invalid.clone());
    fs::write(
        root.join(".minerva/tasks").join(invalid.id.to_string()).join("task.yaml"),
        "schema_version: 99\nid: TSK-000002\ntitle: Broken repair task\n",
    )
    .unwrap();
    fs::remove_file(root.join(".minerva/indexes/tasks.json")).unwrap();
    let output = run(&root, &["--json", "repair"]);
    assert!(output.status.success(), "{output:?}");
    let json = json(&output.stdout);
    assert_eq!(json["result"]["issues"][0]["code"], "invalid_task");
    let contents =
        fs::read_to_string(root.join(".minerva/indexes/tasks.json")).unwrap();
    assert!(contents.contains(&valid.title));
    assert!(!contents.contains(&invalid.title));
    fs::remove_dir_all(root).unwrap();
}

fn json(stdout: &[u8]) -> Value {
    serde_json::from_slice(stdout).unwrap()
}

fn backup_files(root: &std::path::Path) -> Vec<String> {
    fs::read_dir(root.join(".minerva/indexes"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect()
}
