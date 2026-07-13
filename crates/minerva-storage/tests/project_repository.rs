mod support;

use minerva_application::ProjectRepository;
use minerva_domain::MinervaError;
use minerva_storage::FilesystemProjectRepository;
use std::fs;
use support::{sample_project, temp_repo};

#[test]
fn repository_detects_initialization_and_supports_round_trips() {
    let root = temp_repo("project-repository");
    let repo = FilesystemProjectRepository;
    let project = sample_project();
    assert!(!repo.is_initialized(&root));
    repo.save_project(&root, &project).unwrap();
    repo.write_project_instructions(&root, "# Project\n").unwrap();
    assert!(repo.is_initialized(&root));
    assert_eq!(repo.load_project(&root).unwrap(), project);
    assert_eq!(repo.read_project_instructions(&root).unwrap(), "# Project\n");
}

#[test]
fn repository_finds_root_from_nested_directories() {
    let root = temp_repo("project-root");
    let nested = root.join("workspace/src/bin");
    let repo = FilesystemProjectRepository;
    fs::create_dir_all(&nested).unwrap();
    repo.save_project(&root, &sample_project()).unwrap();
    assert_eq!(repo.locate_project_root(&nested).unwrap(), root);
}

#[test]
fn repository_reports_missing_project_root() {
    let root = temp_repo("project-missing-root");
    let repo = FilesystemProjectRepository;
    let error = repo.locate_project_root(&root.join("child")).unwrap_err();
    assert_eq!(error, MinervaError::ProjectNotInitialized);
    assert_eq!(repo.read_project_instructions(&root).unwrap(), "");
}
