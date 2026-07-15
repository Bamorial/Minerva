mod support;

use minerva_application::{TaskCreateRecord, TaskRepository};
use minerva_domain::{DeclarationDocument, RelationshipType};
use minerva_storage::FilesystemTaskRepository;
use std::{thread::sleep, time::Duration};
use support::{relationship, task, temp_repo};

#[test]
fn instruction_updates_do_not_stale_declarations() {
    let root = temp_repo("task-freshness-instructions");
    let repo = FilesystemTaskRepository;
    let task = task(1, "Track declaration freshness");
    repo.create_task(&root, &record(task.clone())).unwrap();
    sleep(Duration::from_millis(20));
    repo.update_task_instructions(
        &root,
        task.id,
        task.version,
        "# Feature\n\nEdited.\n",
    )
    .unwrap();
    let report = report(repo, &root, task.id);
    assert!(report.reasons.is_empty());
}

#[test]
fn relationship_updates_do_not_stale_declarations() {
    let root = temp_repo("task-freshness-relationships");
    let repo = FilesystemTaskRepository;
    let left = task(1, "Track declaration freshness");
    let right = task(2, "Render task status");
    repo.create_task(&root, &record(left.clone())).unwrap();
    repo.create_task(&root, &record(right.clone())).unwrap();
    sleep(Duration::from_millis(20));
    repo.create_relationship(
        &root,
        &relationship(left.id, right.id, RelationshipType::RelatedTo, None),
    )
    .unwrap();
    let report = report(repo, &root, left.id);
    assert!(report.reasons.is_empty());
}

#[test]
fn task_metadata_updates_do_not_stale_declarations() {
    let root = temp_repo("task-freshness-metadata");
    let repo = FilesystemTaskRepository;
    let child = task(1, "Track declaration freshness");
    let parent = task(2, "Parent task");
    repo.create_task(&root, &record(child.clone())).unwrap();
    repo.create_task(&root, &record(parent.clone())).unwrap();
    sleep(Duration::from_millis(20));
    repo.move_task(&root, child.id, Some(parent.id), child.version).unwrap();
    let report = report(repo, &root, child.id);
    assert!(report.reasons.is_empty());
}

fn record(task: minerva_domain::Task) -> TaskCreateRecord {
    TaskCreateRecord {
        task,
        instructions: "# Feature\n".into(),
        declaration: DeclarationDocument::template(),
    }
}

fn report(
    repo: FilesystemTaskRepository,
    root: &std::path::Path,
    task_id: minerva_domain::TaskId,
) -> minerva_domain::DeclarationFreshnessReport {
    let mut probe = repo.read_declaration_freshness(root, task_id).unwrap();
    probe.current_commit_hash = probe.covered_commit_hash.clone();
    probe.evaluate()
}
