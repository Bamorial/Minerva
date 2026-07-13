mod support;

use minerva_application::ProjectRepository;
use minerva_storage::{FilesystemProjectRepository, MinervaLayout, TASK_TYPES};
use std::fs;
use support::temp_repo;

#[test]
fn initialized_task_type_files_match_snapshots() {
    let root = temp_repo("task-type-templates");
    let repo = FilesystemProjectRepository;
    repo.initialize_project(&root, false).unwrap();
    let layout = MinervaLayout::new(&root);
    for (name, expected) in fixtures() {
        let path = layout.task_types_dir().join(name);
        assert_eq!(fs::read_to_string(path).unwrap(), expected);
    }
}

#[test]
fn built_in_task_type_catalog_matches_snapshot_set() {
    let names: Vec<_> = TASK_TYPES.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        vec![
            "feature.md",
            "bug.md",
            "research.md",
            "refactor.md",
            "documentation.md",
            "chore.md",
        ]
    );
}

fn fixtures() -> [(&'static str, &'static str); 6] {
    [
        ("feature.md", include_str!("fixtures/task-types/feature.md")),
        ("bug.md", include_str!("fixtures/task-types/bug.md")),
        ("research.md", include_str!("fixtures/task-types/research.md")),
        ("refactor.md", include_str!("fixtures/task-types/refactor.md")),
        ("documentation.md", include_str!("fixtures/task-types/documentation.md")),
        ("chore.md", include_str!("fixtures/task-types/chore.md")),
    ]
}
