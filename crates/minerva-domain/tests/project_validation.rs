use minerva_domain::{
    ContextPolicy, MinervaError, Project, ProjectId, StatusDefinition, StatusKey,
    StatusTransition, TaskTypeKey,
};
use std::time::UNIX_EPOCH;

#[test]
fn project_rejects_duplicate_statuses() {
    let todo = StatusKey::new("todo").unwrap();
    let result = Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: "Minerva".into(),
        created_at: UNIX_EPOCH,
        default_task_type: TaskTypeKey::new("feature").unwrap(),
        default_status: todo.clone(),
        statuses: vec![
            StatusDefinition::new(todo.clone(), false),
            StatusDefinition::new(todo, true),
        ],
        transitions: vec![],
        context_policy: ContextPolicy::new(12, 2, 24).unwrap(),
    });
    assert!(
        matches!(result, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "statuses")
    );
}

#[test]
fn project_rejects_missing_default_status_and_unknown_transition_targets() {
    let todo = StatusKey::new("todo").unwrap();
    let done = StatusKey::new("done").unwrap();
    let blocked = StatusKey::new("blocked").unwrap();
    let missing_default = Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: "Minerva".into(),
        created_at: UNIX_EPOCH,
        default_task_type: TaskTypeKey::new("feature").unwrap(),
        default_status: blocked.clone(),
        statuses: vec![StatusDefinition::new(todo.clone(), false)],
        transitions: vec![],
        context_policy: ContextPolicy::new(12, 2, 24).unwrap(),
    });
    let unknown_target = Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: "Minerva".into(),
        created_at: UNIX_EPOCH,
        default_task_type: TaskTypeKey::new("feature").unwrap(),
        default_status: todo.clone(),
        statuses: vec![StatusDefinition::new(todo.clone(), false)],
        transitions: vec![StatusTransition::new(todo, done)],
        context_policy: ContextPolicy::new(12, 2, 24).unwrap(),
    });
    assert!(
        matches!(missing_default, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "default_status")
    );
    assert!(
        matches!(unknown_target, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "transitions.to")
    );
}
