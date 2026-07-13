use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

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
    fs::remove_dir_all(root).unwrap();
}

fn run(root: &PathBuf, args: &[&str]) -> std::process::Output {
    Command::new(binary()).args(args).current_dir(root).output().unwrap()
}

fn binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_minerva-cli").unwrap().into()
}

fn temp_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let sequence = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("minerva-cli-{name}-{unique}-{sequence}"));
    fs::create_dir(&dir).unwrap();
    dir
}
