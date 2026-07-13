mod support;

use minerva_application::TaskRepository;
use minerva_domain::{DeclarationActor, DeclarationDocument};
use minerva_storage::{FilesystemTaskRepository, MinervaLayout};
use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn log_command_reports_empty_logs() {
    let root = temp_dir("cli-task-log-empty");
    assert!(run(&root, &["init"]).status.success());
    let task = task(1, "Inspect empty history");
    create_task(&root, task.clone());
    fs::write(MinervaLayout::new(&root).events_file(task.id), "").unwrap();
    let output = run(&root, &["log", &task.id.to_string()]);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        str::from_utf8(&output.stdout).unwrap(),
        format!("{} 0 event(s)\nno events recorded\n", task.id)
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn log_command_renders_chronological_events_and_filters_by_kind() {
    let root = temp_dir("cli-task-log-filter");
    let repo = FilesystemTaskRepository;
    assert!(run(&root, &["init"]).status.success());
    let task = task(1, "Inspect history");
    create_task(&root, task.clone());
    let declaration = format!("{}\nTracked in log.\n", DeclarationDocument::template());
    repo.update_task_declaration(
        &root,
        task.id,
        task.version,
        DeclarationActor::Human,
        None,
        &declaration,
    )
    .unwrap();
    let output = run(
        &root,
        &["log", &task.id.to_string(), "--kind", "task-declaration-updated"],
    );
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.starts_with(&format!(
        "{} 1 event(s) filtered by task-declaration-updated",
        task.id
    )));
    assert!(stdout.contains(" | human | task-declaration-updated | "));
    assert!(!stdout.contains("task-created"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn log_command_json_reports_malformed_lines_without_hiding_valid_entries() {
    let root = temp_dir("cli-task-log-malformed");
    assert!(run(&root, &["init"]).status.success());
    let task = task(1, "Inspect malformed history");
    create_task(&root, task.clone());
    let path = MinervaLayout::new(&root).events_file(task.id);
    let current = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{current}{{bad json\n")).unwrap();
    let output = run(&root, &["--json", "log", &task.id.to_string()]);
    assert!(output.status.success(), "{output:?}");
    let body: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["command"], "log");
    assert_eq!(body["result"]["task"]["id"], task.id.to_string());
    assert_eq!(body["result"]["events"].as_array().unwrap().len(), 1);
    assert_eq!(body["result"]["events"][0]["kind"], "task-created");
    assert_eq!(body["result"]["issues"].as_array().unwrap().len(), 1);
    assert_eq!(body["result"]["issues"][0]["line"], 2);
    fs::remove_dir_all(root).unwrap();
}
