#![allow(dead_code)]

pub mod interactive;

use minerva_application::{TaskCreateRecord, TaskRepository};
use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationDocument, DeclarationMetadata,
    StatusKey, Task, TaskFacts, TaskIdAllocator, TaskPriority, TaskTypeKey,
    TaskVersion,
};
use minerva_storage::FilesystemTaskRepository;
use std::{
    collections::BTreeSet,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

pub fn run(root: &PathBuf, args: &[&str]) -> Output {
    Command::new(binary()).args(args).current_dir(root).output().unwrap()
}

pub fn temp_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let sequence = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("minerva-cli-{name}-{unique}-{sequence}"));
    fs::create_dir(&dir).unwrap();
    dir
}

#[allow(dead_code)]
pub fn write_editor(root: &Path, name: &str, body: &str) -> PathBuf {
    let path = root.join(name);
    fs::write(&path, format!("#!/bin/sh\nset -eu\n{body}")).unwrap();
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();
    path
}

#[allow(dead_code)]
pub fn write_config(root: &Path, editor: &Path) {
    fs::write(root.join(".minerva/config.yaml"), format!(
        "schema_version: 1\neditor: {}\ndefault_priority: Medium\ndefault_tags: []\n",
        editor.display()
    )).unwrap();
}

#[allow(dead_code)]
pub fn create_task(root: &Path, task: Task) {
    FilesystemTaskRepository
        .create_task(
            root,
            &TaskCreateRecord {
                task,
                instructions: "# Feature\n".into(),
                declaration: DeclarationDocument::template(),
            },
        )
        .unwrap();
}

#[allow(dead_code)]
pub fn task(sequence: u32, title: &str) -> Task {
    Task::new(Task {
        schema_version: 1,
        id: TaskIdAllocator::new(sequence - 1).next_id(),
        title: title.into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("backlog").unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: BTreeSet::default(),
        created_at: UNIX_EPOCH,
        updated_at: UNIX_EPOCH,
        completed_at: None,
        version: TaskVersion::initial(),
        declaration: DeclarationMetadata {
            version: 1,
            updated_at: UNIX_EPOCH,
            updated_by: DeclarationActor::Human,
            commit_hash: None,
        },
        facts: TaskFacts::default(),
        archive_state: ArchiveState::Active,
    })
    .unwrap()
}

fn binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_minerva-cli").unwrap().into()
}
