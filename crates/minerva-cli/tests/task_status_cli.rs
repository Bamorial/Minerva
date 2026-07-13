mod support;

use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn status_command_displays_declaration_freshness() {
    let root = temp_dir("cli-task-status");
    assert!(run(&root, &["init"]).status.success());
    let task = task(1, "Inspect declaration freshness");
    create_task(&root, task.clone());
    let output = run(&root, &["status", &task.id.to_string()]);
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("declaration freshness: potentially-stale"));
    assert!(stdout.contains("freshness reasons: missing-covered-commit"));
    fs::remove_dir_all(root).unwrap();
}
