mod support;

use minerva_application::{CreateTaskRequest, TaskCreationService};
use minerva_domain::{MinervaError, TaskPriority, TaskTag};
use std::collections::BTreeSet;
use std::path::Path;
use support::{FakeProjectRepo, FakeTaskRepo, config, project, task_type};

#[test]
fn creation_uses_project_defaults_and_templates() {
    let project_repo = FakeProjectRepo {
        project: project(),
        config: config(),
        task_types: vec![task_type("feature", "# Feature\n")],
    };
    let task_repo = FakeTaskRepo::new(7, Vec::new());
    let result = TaskCreationService::create(
        &project_repo,
        &task_repo,
        Path::new("."),
        CreateTaskRequest {
            title: "Ship creation service".into(),
            task_type: None,
            parent_id: None,
            priority: None,
            tags: None,
        },
    )
    .unwrap();
    assert_eq!(result.task.id.to_string(), "TSK-000008");
    assert_eq!(result.task.priority, TaskPriority::High);
    assert_eq!(result.task.tags, BTreeSet::from([TaskTag::new("release").unwrap()]));
    assert!(result.write_result.event_id.is_some());
    let created = task_repo.created.borrow().clone().unwrap();
    assert_eq!(created.instructions, "# Feature\n");
    assert!(created.declaration.starts_with("# Declaration\n"));
}

#[test]
fn creation_rejects_unknown_task_types_and_missing_parents() {
    let project_repo = FakeProjectRepo {
        project: project(),
        config: config(),
        task_types: vec![task_type("feature", "# Feature\n")],
    };
    let missing_type = TaskCreationService::create(
        &project_repo,
        &FakeTaskRepo::new(0, Vec::new()),
        Path::new("."),
        CreateTaskRequest {
            title: "Ship creation service".into(),
            task_type: Some(minerva_domain::TaskTypeKey::new("bug").unwrap()),
            parent_id: None,
            priority: None,
            tags: None,
        },
    )
    .unwrap_err();
    assert!(
        matches!(missing_type, MinervaError::InvalidConfiguration { key, .. } if key == "task_type")
    );
    let missing_parent = TaskCreationService::create(
        &project_repo,
        &FakeTaskRepo::new(0, Vec::new()),
        Path::new("."),
        CreateTaskRequest {
            title: "Ship creation service".into(),
            task_type: None,
            parent_id: Some("TSK-000001".parse().unwrap()),
            priority: None,
            tags: None,
        },
    )
    .unwrap_err();
    assert!(matches!(missing_parent, MinervaError::TaskNotFound { .. }));
}

impl FakeTaskRepo {
    fn new(last_id: u32, tasks: Vec<minerva_domain::Task>) -> Self {
        let next_id = minerva_domain::TaskIdAllocator::new(last_id).next_id();
        Self { next_id, tasks, created: std::cell::RefCell::new(None) }
    }
}
