mod support;

use minerva_domain::{ArchiveState, StatusKey};
use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn tree_command_renders_deep_hierarchies_and_filters() {
    let root = temp_dir("cli-task-tree");
    assert!(run(&root, &["init"]).status.success());
    let a = task(1, "Root");
    let mut b = task(2, "Child");
    let mut c = task(3, "Grandchild");
    let mut d = task(4, "Great grandchild");
    let mut archived = task(5, "Archived root");
    b.parent_id = Some(a.id);
    c.parent_id = Some(b.id);
    d.parent_id = Some(c.id);
    d.status = StatusKey::new("in-progress").unwrap();
    archived.archive_state = ArchiveState::Archived;
    for task in [&a, &b, &c, &d, &archived] {
        create_task(&root, task.clone());
    }
    let output =
        run(&root, &["tree", "--status", "in-progress", "--archive-state", "all"]);
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("showing 1 matching tasks across 1 roots (total 5)"));
    assert!(stdout.contains(&format!("`-- {} [in-progress] Great grandchild", d.id)));
    assert!(!stdout.contains("Archived root"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn tree_command_supports_json_and_archive_filter() {
    let root = temp_dir("cli-task-tree-json");
    assert!(run(&root, &["init"]).status.success());
    let active = task(1, "Active root");
    let mut archived = task(2, "Archived root");
    let mut child = task(3, "Archived child");
    archived.archive_state = ArchiveState::Archived;
    child.parent_id = Some(archived.id);
    child.archive_state = ArchiveState::Archived;
    for task in [&active, &archived, &child] {
        create_task(&root, task.clone());
    }
    let output = run(&root, &["--json", "tree", "--archive-state", "archived"]);
    assert!(output.status.success(), "{output:?}");
    let body: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["command"], "tree");
    assert_eq!(body["result"]["matched"], 2);
    assert_eq!(body["result"]["roots"][0]["title"], "Archived root");
    assert_eq!(body["result"]["roots"][0]["children"][0]["title"], "Archived child");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn tree_command_reports_hierarchy_cycles() {
    let root = temp_dir("cli-task-tree-cycle");
    assert!(run(&root, &["init"]).status.success());
    let parent = task(1, "Parent");
    let mut child = task(2, "Child");
    child.parent_id = Some(parent.id);
    create_task(&root, parent.clone());
    create_task(&root, child.clone());
    let task_file =
        root.join(".minerva/tasks").join(parent.id.to_string()).join("task.yaml");
    let body = fs::read_to_string(&task_file).unwrap();
    fs::write(
        task_file,
        body.replace("parent_id: null", &format!("parent_id: {}", child.id)),
    )
    .unwrap();
    let output = run(&root, &["tree"]);
    assert_eq!(output.status.code(), Some(15), "{output:?}");
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("hierarchy_cycle"));
    fs::remove_dir_all(root).unwrap();
}
