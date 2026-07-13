#[path = "support/task_transition.rs"]
mod task_transition_support;

use minerva_domain::{MinervaError, StatusKey, TaskTransitionService};
use std::time::{Duration, UNIX_EPOCH};

#[test]
fn default_transition_matrix_accepts_only_configured_edges() {
    let project = task_transition_support::project();
    let cases = [
        ("backlog", "ready", true),
        ("ready", "in-progress", true),
        ("in-progress", "blocked", true),
        ("blocked", "in-progress", true),
        ("in-progress", "review", true),
        ("review", "completed", true),
        ("review", "cancelled", true),
        ("backlog", "completed", false),
    ];
    for (from, to, valid) in cases {
        let result = TaskTransitionService::apply(
            &project,
            &task_transition_support::task(from, None),
            &StatusKey::new(to).unwrap(),
            UNIX_EPOCH + Duration::from_secs(5),
        );
        assert_eq!(result.is_ok(), valid, "{from} -> {to}");
    }
}

#[test]
fn invalid_transitions_return_structured_errors() {
    let result = TaskTransitionService::apply(
        &project(),
        &task_transition_support::task("backlog", None),
        &StatusKey::new("completed").unwrap(),
        UNIX_EPOCH,
    );
    assert!(matches!(
        result,
        Err(MinervaError::InvalidStatusTransition { from, to })
            if from == "backlog" && to == "completed"
    ));
}

fn project() -> minerva_domain::Project {
    task_transition_support::project()
}
