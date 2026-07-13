mod support;

use minerva_application::{TaskCreateRecord, TaskRepository};
use minerva_domain::{DeclarationDocument, StatusKey, TaskTransitionService};
use minerva_storage::{FilesystemTaskRepository, MinervaLayout};
use std::{fs, time::UNIX_EPOCH};
use support::{task, temp_repo};

#[test]
fn transition_task_records_status_event_and_override_flag() {
    let root = temp_repo("task-status-transition");
    let repo = FilesystemTaskRepository;
    let mut task = task(1, "Complete task");
    task.status = StatusKey::new("review").unwrap();
    repo.create_task(
        &root,
        &TaskCreateRecord {
            task: task.clone(),
            instructions: "# Feature\n".into(),
            declaration: DeclarationDocument::template(),
        },
    )
    .unwrap();
    let completed = TaskTransitionService::apply(
        &project(),
        &task,
        &StatusKey::new("completed").unwrap(),
        UNIX_EPOCH,
    )
    .unwrap();
    let result = repo.transition_task(&root, &completed.current, true).unwrap();
    assert_eq!(result.previous_version, Some(task.version));
    let events =
        fs::read_to_string(MinervaLayout::new(&root).events_file(task.id)).unwrap();
    assert!(events.contains("\"kind\":\"task-status-changed\""));
    assert!(events.contains("\"from_status\":\"review\""));
    assert!(events.contains("\"to_status\":\"completed\""));
    assert!(events.contains("\"completion_override\":true"));
}

fn project() -> minerva_domain::Project {
    use minerva_domain::{
        ContextPolicy, Project, ProjectId, StatusDefinition, StatusTransition,
        TaskTypeKey,
    };
    Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: "Minerva".into(),
        created_at: UNIX_EPOCH,
        default_task_type: TaskTypeKey::new("feature").unwrap(),
        default_status: StatusKey::new("review").unwrap(),
        statuses: vec![
            StatusDefinition::new(StatusKey::new("review").unwrap(), false),
            StatusDefinition::new(StatusKey::new("completed").unwrap(), true),
        ],
        transitions: vec![StatusTransition::new(
            StatusKey::new("review").unwrap(),
            StatusKey::new("completed").unwrap(),
        )],
        context_policy: ContextPolicy::strict(),
    })
    .unwrap()
}
