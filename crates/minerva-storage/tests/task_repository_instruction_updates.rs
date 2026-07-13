mod support;

use minerva_application::{TaskCreateRecord, TaskRepository};
use minerva_storage::FilesystemTaskRepository;
use std::time::UNIX_EPOCH;
use support::{task, temp_repo};

#[test]
fn repository_updates_task_metadata_and_records_instruction_events() {
    let root = temp_repo("task-instruction-updates");
    let repo = FilesystemTaskRepository;
    let task = task(1, "Implement instruction editing");
    repo.create_task(
        &root,
        &TaskCreateRecord {
            task: task.clone(),
            instructions: "# Feature\n".into(),
            declaration: "# Declaration\n".into(),
        },
    )
    .unwrap();
    let result = repo
        .update_task_instructions(
            &root,
            task.id,
            task.version,
            "# Feature\n\nEdited.\n",
        )
        .unwrap();
    let updated = repo.read_task(&root, task.id).unwrap();
    assert_eq!(result.previous_version, Some(task.version));
    assert_eq!(result.current_version, task.version.next());
    assert_eq!(updated.version, task.version.next());
    assert!(updated.updated_at > UNIX_EPOCH);
    assert_eq!(updated.declaration.updated_at, UNIX_EPOCH);
    assert!(
        std::fs::read_to_string(
            root.join(".minerva/tasks").join(task.id.to_string()).join("events.jsonl")
        )
        .unwrap()
        .contains("task-instructions-updated")
    );
}
