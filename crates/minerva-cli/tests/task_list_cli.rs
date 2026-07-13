mod support;

use minerva_domain::{ArchiveState, StatusKey, TaskPriority, TaskTag};
use std::{
    collections::BTreeSet,
    fs, str,
    time::{Duration, UNIX_EPOCH},
};
use support::{create_task, run, task, temp_dir};

#[test]
fn list_command_supports_combined_filters_and_priority_sort() {
    let root = temp_dir("cli-task-list");
    assert!(run(&root, &["init"]).status.success());
    let parent = task(1, "Parent task");
    let mut match_one = task(2, "Alpha search match");
    let mut match_two = task(3, "Beta search match");
    let mut archived = task(4, "Archived search match");
    let mut other = task(5, "Gamma skip");
    match_one.parent_id = Some(parent.id);
    match_two.parent_id = Some(parent.id);
    archived.parent_id = Some(parent.id);
    match_one.status = StatusKey::new("in-progress").unwrap();
    match_two.status = StatusKey::new("in-progress").unwrap();
    archived.status = StatusKey::new("in-progress").unwrap();
    match_one.priority = TaskPriority::High;
    match_two.priority = TaskPriority::Urgent;
    archived.priority = TaskPriority::Urgent;
    match_one.tags = tags(["cli"]);
    match_two.tags = tags(["cli"]);
    archived.tags = tags(["cli"]);
    other.tags = tags(["docs"]);
    archived.archive_state = ArchiveState::Archived;
    for (index, task) in
        [&parent, &match_one, &match_two, &archived, &other].into_iter().enumerate()
    {
        let mut item = task.clone();
        item.updated_at = UNIX_EPOCH + Duration::from_secs(index as u64);
        create_task(&root, item);
    }
    let output = run(
        &root,
        &[
            "list",
            "--status",
            "in-progress",
            "--type",
            "feature",
            "--parent",
            &parent.id.to_string(),
            "--tag",
            "cli",
            "--search",
            "search",
            "--sort",
            "priority",
            "--limit",
            "1",
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("showing 1 of 2 matching tasks"));
    assert!(stdout.contains(
        "TSK-000003 Beta search match | in-progress | feature | urgent | active"
    ));
    assert!(stdout.contains("more tasks available; use --offset or --all"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn list_command_supports_json_and_archived_filter() {
    let root = temp_dir("cli-task-list-json");
    assert!(run(&root, &["init"]).status.success());
    let mut active = task(1, "Active task");
    let mut archived = task(2, "Archived task");
    active.tags = tags(["cli"]);
    archived.tags = tags(["cli"]);
    archived.archive_state = ArchiveState::Archived;
    create_task(&root, active);
    create_task(&root, archived);
    let output =
        run(&root, &["--json", "list", "--archive-state", "archived", "--all"]);
    assert!(output.status.success(), "{output:?}");
    let body: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["command"], "list");
    assert_eq!(body["result"]["matched"], 1);
    assert_eq!(body["result"]["tasks"][0]["title"], "Archived task");
    assert_eq!(body["result"]["tasks"][0]["archive_state"], "archived");
    assert_eq!(body["result"]["tasks"][0]["tags"][0], "cli");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn list_command_shows_single_match_with_default_limit() {
    let root = temp_dir("cli-task-list-default-limit");
    assert!(run(&root, &["init"]).status.success());
    create_task(&root, task(1, "Only task"));
    let output = run(&root, &["list"]);
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("showing 1 of 1 matching tasks"));
    assert!(
        stdout.contains("TSK-000001 Only task | backlog | feature | medium | active")
    );
    fs::remove_dir_all(root).unwrap();
}

fn tags(values: [&str; 1]) -> BTreeSet<TaskTag> {
    values.into_iter().map(|value| TaskTag::new(value).unwrap()).collect()
}
