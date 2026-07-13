mod support;

use minerva_application::ProjectRepository;
use minerva_domain::{MinervaError, TaskTypeKey};
use minerva_storage::{FilesystemProjectRepository, MinervaLayout};
use std::fs;
use support::temp_repo;

#[test]
fn repository_loads_built_in_task_types_from_initialized_project() {
    let root = temp_repo("task-types-built-in");
    let repo = FilesystemProjectRepository;
    repo.initialize_project(&root, false).unwrap();
    let task_types = repo.load_task_types(&root).unwrap();
    assert_eq!(task_types.len(), 6);
    assert_eq!(task_types[5].name, TaskTypeKey::new("research").unwrap());
}

#[test]
fn repository_loads_custom_task_types_and_rejects_duplicates() {
    let root = temp_repo("task-types-custom");
    let repo = FilesystemProjectRepository;
    repo.initialize_project(&root, false).unwrap();
    let layout = MinervaLayout::new(&root);
    fs::write(
        layout.task_types_dir().join("spike.md"),
        "---\nname: spike\ndisplay_name: Spike\ndeclaration_requirements:\n  - Record open questions\n---\n# Spike\n\nInvestigate feasibility.\n",
    )
    .unwrap();
    let task_types = repo.load_task_types(&root).unwrap();
    assert!(
        task_types.iter().any(|item| item.name == TaskTypeKey::new("spike").unwrap())
    );
    fs::write(
        layout.task_types_dir().join("duplicate.md"),
        "---\nname: spike\ndisplay_name: Duplicate Spike\n---\n# Duplicate\n\nThis should fail.\n",
    )
    .unwrap();
    let error = repo.load_task_types(&root).unwrap_err();
    match error {
        MinervaError::SchemaError { reason, .. } => {
            assert!(reason.contains("duplicate task type `spike`"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
