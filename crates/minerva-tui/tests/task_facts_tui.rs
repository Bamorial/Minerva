use minerva_application::{ProjectRepository, TaskCreateRecord, TaskRepository};
use minerva_domain::{
    DeclarationDocument, StatusKey, TaskFacts, TaskPriority, TaskResources, TaskTypeKey,
};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::{
    collections::BTreeSet,
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

#[test]
fn tui_renders_task_facts_for_first_task() {
    let root = temp_dir("tui-task-facts");
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    let mut task = task();
    task.facts = TaskFacts {
        modules: vec!["minerva-tui".into()],
        files: vec!["crates/minerva-tui/src/main.rs".into()],
        migrations_required: false,
        feature_flags: vec!["task-facts".into()],
        acceptance_checks: vec!["render task facts".into()],
        resources: TaskResources::default(),
    };
    FilesystemTaskRepository
        .create_task(
            &root,
            &TaskCreateRecord {
                task: task.clone(),
                instructions: "# Feature\n".into(),
                declaration: DeclarationDocument::template(),
            },
        )
        .unwrap();
    let output = Command::new(binary()).current_dir(&root).output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("facts.modules: minerva-tui"));
    assert!(stdout.contains("facts.files: crates/minerva-tui/src/main.rs"));
    fs::remove_dir_all(root).unwrap();
}

fn task() -> minerva_domain::Task {
    let now = UNIX_EPOCH;
    minerva_domain::Task::new(minerva_domain::Task {
        schema_version: 1,
        id: minerva_domain::TaskIdAllocator::new(0).next_id(),
        title: "Render TUI facts".into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("backlog").unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: BTreeSet::default(),
        created_at: now,
        updated_at: now,
        completed_at: None,
        version: minerva_domain::TaskVersion::initial(),
        declaration: minerva_domain::DeclarationMetadata {
            version: 1,
            updated_at: now,
            updated_by: minerva_domain::DeclarationActor::Human,
            commit_hash: None,
        },
        facts: TaskFacts::default(),
        archive_state: minerva_domain::ArchiveState::Active,
    })
    .unwrap()
}

fn temp_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let sequence = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("minerva-tui-{name}-{unique}-{sequence}"));
    fs::create_dir(&dir).unwrap();
    dir
}

fn binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_minerva-tui").unwrap().into()
}
