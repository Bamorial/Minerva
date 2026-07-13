mod support;

use minerva_application::ProjectRepository;
use minerva_domain::{MinervaError, TaskTypeKey};
use minerva_storage::{
    FilesystemProjectRepository, MinervaLayout, SCHEMA_VERSION, agents_md,
    default_config, instructions_md, read_project_config,
};
use std::fs;
use support::temp_repo;

const AGENTS_SNAPSHOT: &str = include_str!("fixtures/agents.md");

#[test]
fn init_creates_expected_layout_and_loadable_project_files() {
    let root = temp_repo("project-init");
    let repo = FilesystemProjectRepository;
    let project = repo.initialize_project(&root, false).unwrap();
    let layout = MinervaLayout::new(&root);
    for path in dirs(&layout) {
        assert!(path.is_dir(), "missing directory {}", path.display());
    }
    for path in files(&layout) {
        assert!(path.is_file(), "missing file {}", path.display());
    }
    assert_eq!(repo.load_project(&root).unwrap(), project);
    assert_eq!(repo.load_project_config(&root).unwrap(), default_config());
    assert_eq!(read_project_config(&layout).unwrap(), default_config());
    assert_eq!(repo.read_project_instructions(&root).unwrap(), instructions_md());
    let task_types = repo.load_task_types(&root).unwrap();
    assert_eq!(task_types.len(), 6);
    assert_eq!(task_types[0].name, TaskTypeKey::new("bug").unwrap());
    assert_eq!(fs::read_to_string(root.join("AGENTS.md")).unwrap(), AGENTS_SNAPSHOT);
    assert_eq!(
        fs::read_to_string(layout.schema_version_file()).unwrap(),
        SCHEMA_VERSION
    );
}

#[test]
fn init_fails_safely_without_force_and_preserves_unrelated_files() {
    let root = temp_repo("project-init-repeat");
    let repo = FilesystemProjectRepository;
    repo.initialize_project(&root, false).unwrap();
    fs::write(root.join("README.md"), "keep me\n").unwrap();
    let error = repo.initialize_project(&root, false).unwrap_err();
    assert_eq!(error, MinervaError::ProjectAlreadyInitialized);
    repo.initialize_project(&root, true).unwrap();
    assert_eq!(fs::read_to_string(root.join("README.md")).unwrap(), "keep me\n");
}

#[test]
fn init_allows_preexisting_agents_file_before_project_is_initialized() {
    let root = temp_repo("project-init-agents");
    let repo = FilesystemProjectRepository;
    fs::write(root.join("AGENTS.md"), "old contents\n").unwrap();
    repo.initialize_project(&root, false).unwrap();
    assert_eq!(repo.load_project_config(&root).unwrap(), default_config());
    assert_eq!(fs::read_to_string(root.join("AGENTS.md")).unwrap(), "old contents\n");
}

#[test]
fn agents_md_matches_snapshot() {
    assert_eq!(agents_md(), AGENTS_SNAPSHOT);
}

fn dirs(layout: &MinervaLayout) -> [std::path::PathBuf; 6] {
    [
        layout.task_types_dir(),
        layout.tasks_dir(),
        layout.indexes_dir(),
        layout.contexts_dir(),
        layout.sessions_dir(),
        layout.locks_dir(),
    ]
}

fn files(layout: &MinervaLayout) -> [std::path::PathBuf; 11] {
    [
        layout.root().join("AGENTS.md"),
        layout.project_file(),
        layout.config_file(),
        layout.instructions_file(),
        layout.schema_version_file(),
        layout.task_types_dir().join("feature.md"),
        layout.task_types_dir().join("bug.md"),
        layout.task_types_dir().join("research.md"),
        layout.task_types_dir().join("refactor.md"),
        layout.task_types_dir().join("documentation.md"),
        layout.task_types_dir().join("chore.md"),
    ]
}
