mod support;

use minerva_application::{MoveTaskRequest, TaskMovementService};
use minerva_domain::{MinervaError, TaskVersion};
use std::path::Path;
use support::{FakeTaskRepo, task};

#[test]
fn movement_checks_parent_existence_and_returns_updated_task() {
    let parent = task(1, "Parent");
    let child = task(2, "Child");
    let repo = FakeTaskRepo::new(2, vec![parent.clone(), child.clone()]);
    let result = TaskMovementService::move_task(
        &repo,
        Path::new("."),
        MoveTaskRequest {
            task_id: child.id,
            new_parent_id: Some(parent.id),
            version: TaskVersion::initial(),
        },
    )
    .unwrap();
    assert_eq!(result.task.parent_id, Some(parent.id));
    assert_eq!(result.write_result.previous_version, Some(TaskVersion::initial()));
    assert_eq!(repo.moved.borrow().clone().unwrap().new_parent_id, Some(parent.id));
}

#[test]
fn movement_rejects_missing_new_parents() {
    let repo = FakeTaskRepo::new(1, vec![task(1, "Child")]);
    let error = TaskMovementService::move_task(
        &repo,
        Path::new("."),
        MoveTaskRequest {
            task_id: "TSK-000001".parse().unwrap(),
            new_parent_id: Some("TSK-000999".parse().unwrap()),
            version: TaskVersion::initial(),
        },
    )
    .unwrap_err();
    assert!(matches!(error, MinervaError::TaskNotFound { .. }));
}
