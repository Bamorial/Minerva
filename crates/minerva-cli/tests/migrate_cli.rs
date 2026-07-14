mod support;

use std::{fs, os::unix::fs::PermissionsExt, path::Path, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn migrate_dry_run_lists_planned_changes_without_writing() {
    let root = legacy_repo("cli-migrate-dry-run");
    let output = run(&root, &["migrate", "--dry-run"]);
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("migrate: would apply 1 step(s)"));
    assert!(stdout.contains("step v0_to_v1 0->1"));
    assert!(!root.join(".minerva/schema-version").exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn migrate_applies_legacy_schema_and_validates_result() {
    let root = legacy_repo("cli-migrate-apply");
    let output = run(&root, &["migrate"]);
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("migrate: applied 1 step(s)"));
    assert!(
        stdout
            .contains("validate project: 0 error(s), 0 warning(s), 1 info finding(s)")
    );
    assert_eq!(
        fs::read_to_string(root.join(".minerva/schema-version")).unwrap(),
        "1\n"
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn migrate_reports_when_project_is_current() {
    let root = temp_dir("cli-migrate-current");
    assert!(run(&root, &["init"]).status.success());
    let output = run(&root, &["migrate"]);
    assert!(output.status.success(), "{output:?}");
    assert!(
        str::from_utf8(&output.stdout).unwrap().contains("migrate: project is current")
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn migrate_failure_keeps_existing_backups() {
    let root = legacy_repo("cli-migrate-failure");
    let locked = root.join(".minerva/tasks/TSK-000002");
    let perms = fs::metadata(&locked).unwrap().permissions();
    fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o555)).unwrap();
    let output = run(&root, &["migrate"]);
    fs::set_permissions(&locked, perms).unwrap();
    assert_eq!(output.status.code(), Some(17), "{output:?}");
    assert!(backup_files(&root).iter().any(|path| path.contains("project.yaml")));
    fs::remove_dir_all(root).unwrap();
}

fn legacy_repo(name: &str) -> std::path::PathBuf {
    let root = temp_dir(name);
    assert!(run(&root, &["init"]).status.success());
    create_task(&root, task(1, "Legacy parent task"));
    create_task(&root, task(2, "Legacy child task"));
    downgrade(&root.join(".minerva/project.yaml"));
    downgrade(&root.join(".minerva/config.yaml"));
    downgrade(&root.join(".minerva/tasks/TSK-000001/task.yaml"));
    downgrade(&root.join(".minerva/tasks/TSK-000002/task.yaml"));
    fs::remove_file(root.join(".minerva/schema-version")).unwrap();
    root
}

fn downgrade(path: &Path) {
    let contents = fs::read_to_string(path).unwrap();
    fs::write(path, contents.replacen("schema_version: 1\n", "", 1)).unwrap();
}

fn backup_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    collect_backups(root, &mut files);
    files
}

fn collect_backups(root: &Path, files: &mut Vec<String>) {
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.file_type().unwrap().is_dir() {
            collect_backups(&path, files);
        } else if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains("schema-migration-backup-")
        {
            files.push(path.display().to_string());
        }
    }
}
