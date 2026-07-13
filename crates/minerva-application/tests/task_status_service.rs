mod support;

use minerva_application::TaskStatusService;
use minerva_domain::StatusKey;
use std::path::Path;
use support::{FakeProjectRepo, FakeTaskRepo, config, project, task, task_type};

#[test]
fn status_transition_persists_changes() {
    let task = task(1, "Move task");
    let repo = FakeTaskRepo::new(1, vec![task.clone()]);
    let result = TaskStatusService::set(
        &project_repo(),
        &repo,
        Path::new("."),
        &task.id.to_string(),
        &StatusKey::new("completed").unwrap(),
    )
    .unwrap();
    assert_eq!(result.task.status.as_str(), "completed");
    assert!(repo.transitioned.borrow().is_some());
}

#[test]
fn reapplying_same_status_is_a_noop() {
    let mut task = task(1, "Keep task");
    task.status = StatusKey::new("completed").unwrap();
    task.completed_at = Some(std::time::UNIX_EPOCH);
    let repo = FakeTaskRepo::new(1, vec![task.clone()]);
    let result = TaskStatusService::set(
        &project_repo(),
        &repo,
        Path::new("."),
        &task.id.to_string(),
        &StatusKey::new("completed").unwrap(),
    )
    .unwrap();
    assert_eq!(result.write_result.event_id, None);
    assert_eq!(result.task, task);
}

fn project_repo() -> FakeProjectRepo {
    FakeProjectRepo {
        project: project(),
        config: config(),
        task_types: vec![task_type("feature", "# Feature\n")],
    }
}
