mod support;

use minerva_domain::Task;
use std::fs;
use support::{create_task, run, task, temp_dir, write_config, write_editor};

#[test]
fn instruction_command_edits_task_instructions_by_id() {
    let (root, task) = setup("cli-task-instruction-id", "Implement docs");
    let output = run(&root, &["instruction", &task.id.to_string()]);
    assert!(output.status.success(), "{output:?}");
    let dir = root.join(".minerva/tasks").join(task.id.to_string());
    assert!(
        fs::read_to_string(dir.join("instructions.md")).unwrap().contains("edited")
    );
    assert!(fs::read_to_string(dir.join("task.yaml")).unwrap().contains("version: 2"));
}

#[test]
fn instruction_command_edits_task_instructions_by_unique_title() {
    let (root, task) = setup("cli-task-instruction-title", "Review docs");
    assert!(run(&root, &["instruction", &task.title]).status.success());
}

#[test]
fn instruction_command_reports_ambiguous_task_references() {
    let root = temp_dir("cli-task-instruction-ambiguous");
    assert!(run(&root, &["init"]).status.success());
    write_config(
        &root,
        &write_editor(&root, "fake-editor.sh", "printf edited >> \"$1\"\n"),
    );
    create_task(&root, task(1, "Implement task repo"));
    create_task(&root, task(2, "Review task repo"));
    let output = run(&root, &["instruction", "task repo"]);
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("ambiguous"));
    assert!(stderr.contains("TSK-000001 Implement task repo"));
}

fn setup(name: &str, title: &str) -> (std::path::PathBuf, Task) {
    let root = temp_dir(name);
    assert!(run(&root, &["init"]).status.success());
    write_config(
        &root,
        &write_editor(&root, "fake-editor.sh", "printf '\\nedited\\n' >> \"$1\"\n"),
    );
    let task = task(1, title);
    create_task(&root, task.clone());
    (root, task)
}
