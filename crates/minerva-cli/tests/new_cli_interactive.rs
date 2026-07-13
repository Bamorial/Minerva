mod support;

use std::fs;
use support::interactive::run_with_input;
use support::{create_task, task, temp_dir, write_config, write_editor};

#[test]
fn new_command_prompts_for_type_and_opens_instructions() {
    let root = temp_dir("cli-new-interactive");
    assert!(run_with_input(&root, &["init"], "").status.success());
    write_config(
        &root,
        &write_editor(&root, "fake-editor.sh", "printf '\\nedited\\n' >> \"$1\"\n"),
    );
    let output = run_with_input(&root, &["new", "Draft roadmap"], "4\n");
    assert!(output.status.success(), "{output:?}");
    let dir = root.join(".minerva/tasks/TSK-000001");
    assert!(
        fs::read_to_string(dir.join("instructions.md")).unwrap().contains("edited")
    );
    assert!(fs::read_to_string(dir.join("task.yaml")).unwrap().contains("version: 2"));
}

#[test]
fn new_command_allows_parent_search_selection() {
    let root = temp_dir("cli-new-parent-search");
    assert!(run_with_input(&root, &["init"], "").status.success());
    create_task(&root, task(1, "Plan platform"));
    create_task(&root, task(2, "Review platform"));
    let output = run_with_input(
        &root,
        &["new", "Add child", "--type", "feature", "--parent", "platform", "--no-edit"],
        "1\n",
    );
    assert!(output.status.success(), "{output:?}");
    let task =
        fs::read_to_string(root.join(".minerva/tasks/TSK-000003/task.yaml")).unwrap();
    assert!(task.contains("parent_id: TSK-000001"));
}
