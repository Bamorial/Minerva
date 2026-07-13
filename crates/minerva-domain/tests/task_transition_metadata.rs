#[path = "support/task_transition.rs"]
mod task_transition_support;

use minerva_domain::{StatusKey, TaskTransitionService};
use std::time::{Duration, UNIX_EPOCH};

#[test]
fn reapplying_same_status_is_a_noop() {
    let task = task_transition_support::task("in-progress", None);
    let outcome = TaskTransitionService::apply(
        &task_transition_support::project(),
        &task,
        &StatusKey::new("in-progress").unwrap(),
        UNIX_EPOCH + Duration::from_secs(10),
    )
    .unwrap();
    assert!(!outcome.changed);
    assert_eq!(outcome.previous, outcome.current);
}

#[test]
fn completion_metadata_is_set_and_cleared_by_reopen() {
    let completed = TaskTransitionService::apply(
        &task_transition_support::project(),
        &task_transition_support::task("review", None),
        &StatusKey::new("completed").unwrap(),
        UNIX_EPOCH + Duration::from_secs(10),
    )
    .unwrap();
    assert_eq!(
        completed.current.completed_at,
        Some(UNIX_EPOCH + Duration::from_secs(10))
    );
    let reopened = TaskTransitionService::apply(
        &task_transition_support::project(),
        &completed.current,
        &StatusKey::new("in-progress").unwrap(),
        UNIX_EPOCH + Duration::from_secs(20),
    )
    .unwrap();
    assert_eq!(reopened.current.completed_at, None);
    assert_eq!(reopened.current.version, completed.current.version.next());
}
