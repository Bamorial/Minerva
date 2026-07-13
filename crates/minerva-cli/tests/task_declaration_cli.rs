mod support;

use std::fs;
use support::{create_task, run, task, temp_dir, write_config, write_editor};

#[test]
fn declaration_command_edits_task_declaration_by_id() {
    let (root, task) = setup(
        "cli-task-declaration-id",
        "Implement docs",
        "printf '\\nUpdated.\\n' >> \"$1\"\n",
    );
    let output = run(&root, &["declaration", &task.id.to_string()]);
    assert!(output.status.success(), "{output:?}");
    let dir = root.join(".minerva/tasks").join(task.id.to_string());
    assert!(
        fs::read_to_string(dir.join("declaration.md")).unwrap().contains("Updated.")
    );
    let yaml = fs::read_to_string(dir.join("task.yaml")).unwrap();
    assert!(yaml.contains("version: 2"));
    assert!(yaml.contains("updated_by: Human"));
}

#[test]
fn declaration_command_does_not_bump_version_when_unchanged() {
    let (root, task) = setup("cli-task-declaration-noop", "Review docs", ":\n");
    let output = run(&root, &["declaration", &task.id.to_string()]);
    assert!(output.status.success(), "{output:?}");
    let dir = root.join(".minerva/tasks").join(task.id.to_string());
    assert!(fs::read_to_string(dir.join("task.yaml")).unwrap().contains("version: 1"));
    assert!(
        !fs::read_to_string(dir.join("events.jsonl"))
            .unwrap()
            .contains("task-declaration-updated")
    );
}

fn setup(
    name: &str,
    title: &str,
    body: &str,
) -> (std::path::PathBuf, minerva_domain::Task) {
    let root = temp_dir(name);
    assert!(run(&root, &["init"]).status.success());
    write_config(&root, &write_editor(&root, "fake-editor.sh", body));
    let task = task(1, title);
    create_task(&root, task.clone());
    (root, task)
}
