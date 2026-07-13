mod support;

use minerva_storage::MinervaLayout;
use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn status_command_updates_task_status() {
    let root = temp_dir("cli-task-status");
    assert!(run(&root, &["init"]).status.success());
    let task = task(1, "Advance task");
    create_task(&root, task.clone());
    let output = run(&root, &["status", &task.id.to_string(), "in-progress"]);
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("TSK-000001 -> in-progress"));
    let show = run(&root, &["show", &task.id.to_string()]);
    assert!(str::from_utf8(&show.stdout).unwrap().contains("status: in-progress"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn complete_and_reopen_commands_follow_default_transitions() {
    let root = temp_dir("cli-task-complete");
    assert!(run(&root, &["init"]).status.success());
    let task = task(1, "Ship feature");
    create_task(&root, task.clone());
    assert!(
        run(&root, &["status", &task.id.to_string(), "in-progress"]).status.success()
    );
    let path = MinervaLayout::new(&root).declaration_file(task.id);
    fs::write(&path, declaration()).unwrap();
    let completed = run(&root, &["complete", &task.id.to_string()]);
    assert!(completed.status.success(), "{completed:?}");
    assert!(str::from_utf8(&completed.stdout).unwrap().contains("-> completed"));
    let reopened = run(&root, &["reopen", &task.id.to_string()]);
    assert!(reopened.status.success(), "{reopened:?}");
    let show = run(&root, &["show", &task.id.to_string()]);
    let stdout = str::from_utf8(&show.stdout).unwrap();
    assert!(stdout.contains("status: in-progress"));
    assert!(stdout.contains("completed_at: none"));
    fs::remove_dir_all(root).unwrap();
}

fn declaration() -> String {
    minerva_domain::DeclarationDocument::template()
        .replace("## Current State\n", "## Current State\nReady for completion.\n")
        .replace(
            "## Completed Work\n",
            "## Completed Work\nImplemented status commands.\n",
        )
        .replace("## Verification\n", "## Verification\ncargo test\n")
}
