mod support;

use minerva_application::{TaskCreateRecord, TaskRepository};
use minerva_domain::{DeclarationActor, DeclarationDocument};
use minerva_storage::FilesystemTaskRepository;
use std::time::UNIX_EPOCH;
use support::{task, temp_repo};

#[test]
fn repository_updates_declaration_metadata_and_records_events() {
    let root = temp_repo("task-declaration-updates");
    let repo = FilesystemTaskRepository;
    let task = task(1, "Implement declaration editing");
    repo.create_task(
        &root,
        &TaskCreateRecord {
            task: task.clone(),
            instructions: "# Feature\n".into(),
            declaration: DeclarationDocument::template(),
        },
    )
    .unwrap();
    let contents = format!("{}\nUpdated.\n", DeclarationDocument::template());
    let result = repo
        .update_task_declaration(
            &root,
            task.id,
            task.version,
            DeclarationActor::Human,
            Some("abc123".into()),
            &contents,
        )
        .unwrap();
    let updated = repo.read_task(&root, task.id).unwrap();
    assert_eq!(result.previous_version, Some(task.version));
    assert_eq!(result.current_version, task.version.next());
    assert_eq!(updated.version, task.version.next());
    assert_eq!(updated.declaration.version, 2);
    assert_eq!(updated.declaration.updated_by, DeclarationActor::Human);
    assert_eq!(updated.declaration.commit_hash.as_deref(), Some("abc123"));
    assert!(updated.updated_at > UNIX_EPOCH);
    assert!(updated.declaration.updated_at > UNIX_EPOCH);
    assert!(
        std::fs::read_to_string(
            root.join(".minerva/tasks").join(task.id.to_string()).join("events.jsonl")
        )
        .unwrap()
        .contains("task-declaration-updated")
    );
}
