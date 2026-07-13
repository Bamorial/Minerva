mod support;

use std::fs;
use support::{run, temp_dir};

#[test]
fn init_command_creates_minerva_project_structure() {
    let root = temp_dir("cli-init");
    let output = run(&root, &["init"]);
    assert!(output.status.success(), "{output:?}");
    assert!(root.join("AGENTS.md").is_file());
    assert!(root.join(".minerva/project.yaml").is_file());
    assert!(root.join(".minerva/task-types/feature.md").is_file());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn init_command_requires_force_for_repeat_runs() {
    let root = temp_dir("cli-repeat");
    assert!(run(&root, &["init"]).status.success());
    fs::write(root.join("README.md"), "keep me\n").unwrap();
    let repeated = run(&root, &["init"]);
    assert!(!repeated.status.success(), "{repeated:?}");
    let forced = run(&root, &["init", "--force"]);
    assert!(forced.status.success(), "{forced:?}");
    assert_eq!(fs::read_to_string(root.join("README.md")).unwrap(), "keep me\n");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn init_command_allows_preexisting_agents_file() {
    let root = temp_dir("cli-agents");
    fs::write(root.join("AGENTS.md"), "old contents\n").unwrap();
    let output = run(&root, &["init"]);
    assert!(output.status.success(), "{output:?}");
    assert!(root.join(".minerva/project.yaml").is_file());
    assert_eq!(fs::read_to_string(root.join("AGENTS.md")).unwrap(), "old contents\n");
    fs::remove_dir_all(root).unwrap();
}
