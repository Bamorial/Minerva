mod support;

use minerva_application::ProjectRepository;
use minerva_storage::{
    FilesystemProjectRepository, MinervaLayout, initialize_project, read_project,
    read_project_config, read_relationships, read_task,
};
use std::{fs, path::Path};
use support::temp_repo;

#[test]
fn current_schema_projects_report_no_migration_work() {
    let root = temp_repo("schema-current");
    initialize_project(&root, false).unwrap();
    let result =
        FilesystemProjectRepository.migrate_project_state(&root, true).unwrap();
    assert!(result.is_current());
    assert_eq!(result.start_version, 1);
}

#[test]
fn dry_run_reports_legacy_changes_without_writing() {
    let root = legacy_repo("schema-dry-run");
    let before = fs::read_to_string(root.join(".minerva/project.yaml")).unwrap();
    let result =
        FilesystemProjectRepository.migrate_project_state(&root, true).unwrap();
    assert_eq!(result.start_version, 0);
    assert_eq!(result.steps.len(), 1);
    assert_eq!(result.steps[0].name, "v0_to_v1");
    assert!(result.steps[0].operations.len() >= 5);
    assert_eq!(fs::read_to_string(root.join(".minerva/project.yaml")).unwrap(), before);
    assert!(!root.join(".minerva/schema-version").exists());
}

#[test]
fn legacy_fixture_migrates_to_current_schema_with_backups() {
    let root = legacy_repo("schema-apply");
    let result =
        FilesystemProjectRepository.migrate_project_state(&root, false).unwrap();
    let layout = MinervaLayout::new(&root);
    assert_eq!(result.start_version, 0);
    assert_eq!(fs::read_to_string(layout.schema_version_file()).unwrap(), "1\n");
    assert_eq!(read_project(&layout).unwrap().schema_version, 1);
    assert_eq!(read_project_config(&layout).unwrap().schema_version, 1);
    assert_eq!(
        read_task(&layout, "TSK-000001".parse().unwrap()).unwrap().schema_version,
        1
    );
    assert_eq!(
        read_task(&layout, "TSK-000002".parse().unwrap()).unwrap().schema_version,
        1
    );
    assert_eq!(
        read_relationships(&layout, "TSK-000001".parse().unwrap()).unwrap()[0]
            .schema_version,
        1
    );
    assert!(result.steps[0].operations.iter().any(|item| item.backup_path.is_some()));
}

#[test]
fn applied_migrations_are_safe_to_rerun() {
    let root = legacy_repo("schema-rerun");
    FilesystemProjectRepository.migrate_project_state(&root, false).unwrap();
    let rerun =
        FilesystemProjectRepository.migrate_project_state(&root, false).unwrap();
    assert!(rerun.is_current());
    assert_eq!(rerun.start_version, 1);
}

fn legacy_repo(name: &str) -> std::path::PathBuf {
    let root = temp_repo(name);
    copy_dir(&support::fixture("schema-v0"), &root);
    root
}

fn copy_dir(from: &Path, to: &Path) {
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            fs::create_dir_all(&target).unwrap();
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
