mod support;

use minerva_application::{CompleteTaskRequest, TaskCompletionService};
use minerva_domain::{MinervaError, TaskVersion};
use std::path::Path;
use support::{FakeProjectRepo, FakeTaskRepo, config, project, task, task_type};

#[test]
fn completion_requires_declaration_handoff_sections() {
    let task = task(1, "Finish validation");
    let repo = FakeTaskRepo::new(1, vec![task.clone()]);
    let error = TaskCompletionService::complete(
        &project_repo(),
        &repo,
        Path::new("."),
        CompleteTaskRequest {
            task_id: task.id,
            version: TaskVersion::initial(),
            allow_declaration_override: false,
        },
    )
    .unwrap_err();
    assert!(matches!(
        error,
        MinervaError::InvalidConfiguration { key, .. } if key == "declaration.completion"
    ));
}

#[test]
fn completion_persists_status_change_when_declaration_is_valid() {
    let task = task(1, "Finish validation");
    let repo = FakeTaskRepo::with_declaration(1, vec![task.clone()], declaration());
    let result = TaskCompletionService::complete(
        &project_repo(),
        &repo,
        Path::new("."),
        CompleteTaskRequest {
            task_id: task.id,
            version: TaskVersion::initial(),
            allow_declaration_override: false,
        },
    )
    .unwrap();
    assert_eq!(result.task.status.as_str(), "completed");
    assert_eq!(result.write_result.previous_version, Some(TaskVersion::initial()));
    assert_eq!(repo.transitioned.borrow().as_ref().unwrap().1, false);
}

#[test]
fn completion_override_bypasses_validation_and_is_audited() {
    let task = task(1, "Finish validation");
    let repo = FakeTaskRepo::new(1, vec![task.clone()]);
    let result = TaskCompletionService::complete(
        &project_repo(),
        &repo,
        Path::new("."),
        CompleteTaskRequest {
            task_id: task.id,
            version: TaskVersion::initial(),
            allow_declaration_override: true,
        },
    )
    .unwrap();
    assert_eq!(result.task.status.as_str(), "completed");
    assert_eq!(repo.transitioned.borrow().as_ref().unwrap().1, true);
}

fn project_repo() -> FakeProjectRepo {
    FakeProjectRepo {
        project: project(),
        config: config(),
        task_types: vec![task_type("feature", "# Feature\n")],
    }
}

fn declaration() -> String {
    minerva_domain::DeclarationDocument::template()
        .replace("## Current State\n", "## Current State\nComplete.\n")
        .replace("## Completed Work\n", "## Completed Work\nAdded validation.\n")
        .replace("## Verification\n", "## Verification\ncargo test\n")
}
