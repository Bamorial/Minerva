mod support;

use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn show_command_supports_json_and_optional_sections() {
    let root = temp_dir("cli-task-show-json");
    assert!(run(&root, &["init"]).status.success());
    let task = task(1, "Inspect full task");
    create_task(&root, task.clone());
    let dir = root.join(".minerva/tasks").join(task.id.to_string());
    fs::write(dir.join("instructions.md"), "# Feature\n\nInspect all data.\n").unwrap();
    fs::write(
        dir.join("declaration.md"),
        "# Declaration\n\n## Objective\nInspect all data.\n\n## Current State\nReady.\n\n## Completed Work\n\n## Remaining Work\n\n## Decisions\n\n## Risks\n\n## Verification\n\n## Open Questions\n",
    )
    .unwrap();
    let output = run(
        &root,
        &["--json", "show", &task.id.to_string(), "--instructions", "--declaration"],
    );
    assert!(output.status.success(), "{output:?}");
    let body: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["command"], "show");
    assert_eq!(body["result"]["task"]["id"], task.id.to_string());
    assert_eq!(
        body["result"]["task"]["instructions"],
        "# Feature\n\nInspect all data.\n"
    );
    assert_eq!(
        body["result"]["task"]["declaration"],
        "# Declaration\n\n## Objective\nInspect all data.\n\n## Current State\nReady.\n\n## Completed Work\n\n## Remaining Work\n\n## Decisions\n\n## Risks\n\n## Verification\n\n## Open Questions\n"
    );
    assert_eq!(
        body["result"]["task"]["declaration_freshness"]["status"],
        "potentially-stale"
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn show_command_reports_missing_tasks_with_structured_error() {
    let root = temp_dir("cli-task-show-missing");
    assert!(run(&root, &["init"]).status.success());
    let output = run(&root, &["--json", "show", "TSK-000099"]);
    assert_eq!(output.status.code(), Some(12), "{output:?}");
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("\"ok\":false"));
    assert!(stderr.contains("\"code\":\"task_not_found\""));
    fs::remove_dir_all(root).unwrap();
}
