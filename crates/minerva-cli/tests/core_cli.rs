mod support;

use std::{fs, process::Command, str};
use support::{create_task, run, task, temp_dir};

#[test]
fn help_includes_global_options_and_exit_codes() {
    let root = temp_dir("cli-help");
    let output = run(&root, &["--help"]);
    assert!(output.status.success(), "{output:?}");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("Minerva command line interface"));
    assert!(stdout.contains("--root <PATH>"));
    assert!(stdout.contains("--json"));
    assert!(stdout.contains("--quiet"));
    assert!(stdout.contains("--verbose"));
    assert!(stdout.contains("show"));
    assert!(stdout.contains("complete"));
    assert!(stdout.contains("reopen"));
    assert!(stdout.contains("Exit codes:"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn version_flag_reports_workspace_version() {
    let root = temp_dir("cli-version");
    let output = run(&root, &["--version"]);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(str::from_utf8(&output.stdout).unwrap().trim(), "minerva 0.1.0");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn quiet_suppresses_success_output() {
    let root = temp_dir("cli-quiet");
    assert!(run(&root, &["init"]).status.success());
    let output = run(&root, &["--quiet", "rebuild"]);
    assert!(output.status.success(), "{output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn json_errors_use_stable_exit_codes_and_root_override() {
    let root = temp_dir("cli-json-error");
    assert!(run(&root, &["init"]).status.success());
    create_task(&root, task(1, "Inspect status output"));
    let other = temp_dir("cli-json-cwd");
    let output = Command::new(std::env::var_os("CARGO_BIN_EXE_minerva-cli").unwrap())
        .args([
            "--root",
            root.to_str().unwrap(),
            "--json",
            "status",
            "TSK-000099",
            "in-progress",
        ])
        .current_dir(&other)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(12), "{output:?}");
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("\"ok\":false"));
    assert!(stderr.contains("\"code\":\"task_not_found\""));
    assert!(stderr.contains("\"exit_code\":12"));
    fs::remove_dir_all(root).unwrap();
    fs::remove_dir_all(other).unwrap();
}
