mod support;

use serde_json::Value;
use std::fs;
use support::{create_task, run, task, temp_dir};

#[test]
fn new_command_creates_task_from_flags_and_returns_stable_json() {
    let root = temp_dir("cli-new-json");
    assert!(run(&root, &["init"]).status.success());
    let parent = task(1, "Parent task");
    create_task(&root, parent.clone());
    let output = run(
        &root,
        &[
            "--json",
            "new",
            "Ship json output",
            "--type",
            "bug",
            "--parent",
            &parent.id.to_string(),
            "--priority",
            "urgent",
            "--tags",
            "release,cli",
            "--no-edit",
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let body: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["command"], "new");
    assert_eq!(body["result"]["task"]["title"], "Ship json output");
    assert_eq!(body["result"]["task"]["task_type"], "bug");
    assert_eq!(body["result"]["task"]["parent_id"], parent.id.to_string());
    assert_eq!(body["result"]["task"]["priority"], "Urgent");
    assert_eq!(body["result"]["task"]["tags"][0], "cli");
    assert_eq!(body["result"]["task"]["tags"][1], "release");
    assert_eq!(body["result"]["instructions"]["opened"], false);
    let dir = root
        .join(".minerva/tasks")
        .join(body["result"]["task"]["id"].as_str().unwrap());
    assert!(dir.join("task.yaml").is_file());
    assert!(dir.join("instructions.md").is_file());
    assert!(fs::read_to_string(dir.join("task.yaml")).unwrap().contains("version: 1"));
}
