mod support;

use minerva_storage::MinervaLayout;
use std::{fs, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn delete_command_removes_a_task_tree_recursively() {
    let root = temp_dir("cli-delete");
    assert!(run(&root, &["init"]).status.success());
    let parent = task(1, "Parent");
    let mut child = task(2, "Child");
    let survivor = task(3, "Survivor");
    child.parent_id = Some(parent.id);
    create_task(&root, parent.clone());
    create_task(&root, child.clone());
    create_task(&root, survivor.clone());
    let output = run(&root, &["delete", &parent.id.to_string()]);
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("deleted 2 task(s)"));
    let layout = MinervaLayout::new(&root);
    assert!(!layout.task_dir(parent.id).exists());
    assert!(!layout.task_dir(child.id).exists());
    assert!(layout.task_dir(survivor.id).exists());
    fs::remove_dir_all(root).unwrap();
}
