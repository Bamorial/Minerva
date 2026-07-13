mod support;

use minerva_application::TaskRepository;
use minerva_domain::{DeclarationDocument, MinervaError, TaskEventKind};
use minerva_storage::{FilesystemTaskRepository, MinervaLayout, read_task_events};
use std::fs;
use support::{create_record, task, temp_repo};

#[test]
fn repository_event_log_preserves_append_order() {
    let root = temp_repo("task-repository-events-order");
    let repo = FilesystemTaskRepository;
    let task = task(1, "Track event ordering");
    repo.create_task(&root, &create_record(task.clone())).unwrap();
    repo.update_task_instructions(&root, task.id, task.version, "# Feature\n\nEdit\n")
        .unwrap();
    let moved = repo.read_task(&root, task.id).unwrap();
    let archived = repo.archive_task(&root, moved.id, moved.version).unwrap();
    assert!(archived.event_id.is_some());
    let events = read_task_events(&MinervaLayout::new(&root), task.id).unwrap();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].kind, TaskEventKind::TaskCreated);
    assert_eq!(events[1].kind, TaskEventKind::TaskInstructionsUpdated);
    assert_eq!(events[2].kind, TaskEventKind::TaskArchived);
}

#[test]
fn malformed_event_lines_surface_schema_errors() {
    let root = temp_repo("task-repository-events-invalid");
    let repo = FilesystemTaskRepository;
    let task = task(1, "Validate event log");
    repo.create_task(
        &root,
        &minerva_application::TaskCreateRecord {
            task: task.clone(),
            instructions: "# Feature\n".into(),
            declaration: DeclarationDocument::template(),
        },
    )
    .unwrap();
    let path = MinervaLayout::new(&root).events_file(task.id);
    fs::write(&path, "{bad json\n").unwrap();
    let error = read_task_events(&MinervaLayout::new(&root), task.id).unwrap_err();
    assert!(
        matches!(error, MinervaError::SchemaError { reason, .. } if reason.contains("line 1"))
    );
}

#[test]
fn lenient_log_reader_keeps_valid_entries_and_reports_invalid_lines() {
    let root = temp_repo("task-repository-events-lenient");
    let repo = FilesystemTaskRepository;
    let task = task(1, "Read malformed history");
    repo.create_task(&root, &create_record(task.clone())).unwrap();
    let path = MinervaLayout::new(&root).events_file(task.id);
    let current = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{current}{{bad json\n")).unwrap();
    let log = repo.read_task_log(&root, task.id).unwrap();
    assert_eq!(log.events.len(), 1);
    assert_eq!(log.events[0].kind, TaskEventKind::TaskCreated);
    assert_eq!(log.issues.len(), 1);
    assert_eq!(log.issues[0].line, 2);
}
