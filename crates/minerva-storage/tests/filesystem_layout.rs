use minerva_domain::TaskId;
use minerva_storage::{LayoutClass, MinervaLayout, project_layout, task_layout};
use std::num::NonZeroU32;
use std::path::PathBuf;

#[test]
fn project_layout_distinguishes_canonical_derived_and_operational_entries() {
    let canonical = project_layout()
        .iter()
        .filter(|entry| entry.class == LayoutClass::Canonical)
        .count();
    let derived = project_layout()
        .iter()
        .filter(|entry| entry.class == LayoutClass::Derived)
        .count();
    let operational = project_layout()
        .iter()
        .filter(|entry| entry.class == LayoutClass::Operational)
        .count();
    assert_eq!((canonical, derived, operational), (6, 3, 1));
}

#[test]
fn task_layout_uses_immutable_task_ids_instead_of_titles() {
    let task_id = task_id(42);
    let layout = MinervaLayout::new(PathBuf::from("repo root"));
    assert_eq!(
        layout.task_dir(task_id),
        expected(&["repo root", ".minerva", "tasks", "TSK-000042"])
    );
    assert!(
        !layout.task_dir(task_id).to_string_lossy().contains("recursive-task-support")
    );
}

#[test]
fn helper_paths_cover_project_task_and_lock_files() {
    let task_id = task_id(7);
    let layout = MinervaLayout::new("/tmp/minerva");
    assert_eq!(
        layout.project_file(),
        expected(&["/tmp/minerva", ".minerva", "project.yaml"])
    );
    assert_eq!(
        layout.events_file(task_id),
        expected(&["/tmp/minerva", ".minerva", "tasks", "TSK-000007", "events.jsonl"])
    );
    assert_eq!(
        layout.relationships_file(task_id),
        expected(&[
            "/tmp/minerva",
            ".minerva",
            "tasks",
            "TSK-000007",
            "relationships.yaml",
        ])
    );
    assert_eq!(
        layout.project_lock_file(),
        expected(&["/tmp/minerva", ".minerva", "locks", "project.lock"])
    );
    assert_eq!(
        layout.task_lock_file(task_id),
        expected(&["/tmp/minerva", ".minerva", "locks", "TSK-000007.lock"])
    );
}

#[test]
fn task_layout_documents_all_canonical_task_files() {
    let paths: Vec<_> = task_layout().iter().map(|entry| entry.relative_path).collect();
    assert_eq!(
        paths,
        vec![
            "tasks/<task-id>/task.yaml",
            "tasks/<task-id>/instructions.md",
            "tasks/<task-id>/declaration.md",
            "tasks/<task-id>/notes.md",
            "tasks/<task-id>/relationships.yaml",
            "tasks/<task-id>/events.jsonl",
        ]
    );
}

fn task_id(sequence: u32) -> TaskId {
    TaskId::from_sequence(
        NonZeroU32::new(sequence).expect("test task IDs are positive"),
    )
}

fn expected(parts: &[&str]) -> PathBuf {
    parts.iter().collect()
}
