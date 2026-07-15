mod support;

use minerva_application::TaskRepository;
use minerva_domain::{
    DeclarationActor, DeclarationDocument, Relationship, RelationshipId,
    RelationshipType, Task,
};
use minerva_storage::FilesystemTaskRepository;
use std::{fs, str, time::UNIX_EPOCH};
use support::{create_task, run, task, temp_dir};

#[test]
fn context_command_prints_deterministic_markdown() {
    let root = temp_dir("cli-context-markdown");
    let target = setup_repo(&root);
    let first = run(&root, &["context", &target.id.to_string()]);
    let second = run(&root, &["context", &target.id.to_string()]);
    assert!(first.status.success(), "{first:?}");
    assert_eq!(first.stdout, second.stdout);
    let stdout = str::from_utf8(&first.stdout).unwrap();
    assert!(stdout.contains("## Target Metadata and Facts"));
    assert!(!stdout.contains("## Context Manifest Summary"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn context_command_writes_markdown_to_a_file() {
    let root = temp_dir("cli-context-markdown-file");
    let target = setup_repo(&root);
    let path = root.join("compiled/context.md");
    let output = run(
        &root,
        &["context", &target.id.to_string(), "--output", path.to_str().unwrap()],
    );
    assert!(output.status.success(), "{output:?}");
    assert!(str::from_utf8(&output.stdout).unwrap().contains("wrote context to"));
    assert!(!fs::read_to_string(path).unwrap().contains("## Context Manifest Summary"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn context_command_supports_json_and_explain() {
    let root = temp_dir("cli-context-json");
    let target = setup_repo(&root);
    let output = run(
        &root,
        &["context", &target.id.to_string(), "--format", "json", "--explain"],
    );
    assert!(output.status.success(), "{output:?}");
    let body: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["format"], "json");
    assert_eq!(body["task_ref"], target.id.to_string());
    assert_eq!(body["explain"]["included_tasks"][0]["reason"]["kind"], "target");
    assert!(body["content"].as_str().unwrap().contains("## Target Metadata and Facts"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn context_command_supports_budgeted_json_file_output() {
    let root = temp_dir("cli-context-budget");
    let target = setup_repo(&root);
    let full = run(&root, &["context", &target.id.to_string(), "--format", "json"]);
    assert!(full.status.success(), "{full:?}");
    let full_body: serde_json::Value = serde_json::from_slice(&full.stdout).unwrap();
    let total = full_body["manifest"]["total_estimated_tokens"].as_u64().unwrap();
    let dependency = full_body["manifest"]["included"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["source"] == "dependency_declarations")
        .unwrap()["estimated_tokens"]
        .as_u64()
        .unwrap();
    let budget = (total - dependency).to_string();
    let path = root.join("compiled/context.json");
    let output = run(
        &root,
        &[
            "context",
            &target.id.to_string(),
            "--format",
            "json",
            "--budget",
            &budget,
            "--explain",
            "--output",
            path.to_str().unwrap(),
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let body: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(body["budget"], total - dependency);
    assert_eq!(body["manifest"]["excluded"][0]["source"], "dependency_declarations");
    assert_eq!(
        body["explain"]["excluded_sections"][0]["source"],
        "dependency_declarations"
    );
    assert!(!body["content"].as_str().unwrap().contains("## Dependency Declarations"));
    fs::remove_dir_all(root).unwrap();
}

fn setup_repo(root: &std::path::Path) -> Task {
    assert!(run(&root.to_path_buf(), &["init"]).status.success());
    let target = task(1, "Target");
    let dependency = task(2, "Dependency");
    create_task(root, target.clone());
    create_task(root, dependency.clone());
    write_instructions(root, &target, "# Feature\n\nCompile context.\n");
    write_instructions(
        root,
        &dependency,
        "# Feature\n\nLong dependency notes.\n\nOne.\nTwo.\nThree.\nFour.\nFive.\n",
    );
    relate(root, target.id, dependency.id);
    refresh_declaration(root, target.id, "Target context is current.");
    refresh_declaration(root, dependency.id, "Dependency context is current.");
    target
}

fn write_instructions(root: &std::path::Path, task: &Task, body: &str) {
    let path =
        root.join(".minerva/tasks").join(task.id.to_string()).join("instructions.md");
    fs::write(path, body).unwrap();
}

fn relate(
    root: &std::path::Path,
    source: minerva_domain::TaskId,
    target: minerva_domain::TaskId,
) {
    FilesystemTaskRepository
        .create_relationship(
            root,
            &Relationship::new(Relationship {
                schema_version: 1,
                id: RelationshipId::new(),
                source_task: source,
                target_task: target,
                relationship_type: RelationshipType::DependsOn,
                reason: None,
                created_at: UNIX_EPOCH,
            })
            .unwrap(),
        )
        .unwrap();
}

fn refresh_declaration(
    root: &std::path::Path,
    task_id: minerva_domain::TaskId,
    body: &str,
) {
    let task = FilesystemTaskRepository.read_task(root, task_id).unwrap();
    let declaration = DeclarationDocument::template()
        .replace("## Objective\n", &format!("## Objective\n{body}\n"));
    FilesystemTaskRepository
        .update_task_declaration(
            root,
            task_id,
            task.version,
            DeclarationActor::Human,
            None,
            &declaration,
        )
        .unwrap();
}
