use minerva_domain::{
    ContextPolicy, Project, ProjectId, StatusDefinition, StatusKey, StatusTransition,
    TaskTypeKey,
};
use std::time::UNIX_EPOCH;

#[test]
fn project_accepts_valid_status_and_transition_configuration() {
    let todo = StatusKey::new("todo").unwrap();
    let doing = StatusKey::new("doing").unwrap();
    let done = StatusKey::new("done").unwrap();
    let project = Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: "Minerva".into(),
        created_at: UNIX_EPOCH,
        default_task_type: TaskTypeKey::new("feature").unwrap(),
        default_status: todo.clone(),
        statuses: vec![
            StatusDefinition::new(todo.clone(), false),
            StatusDefinition::new(doing.clone(), false),
            StatusDefinition::new(done.clone(), true),
        ],
        transitions: vec![
            StatusTransition::new(todo.clone(), doing.clone()),
            StatusTransition::new(doing.clone(), done.clone()),
        ],
        context_policy: ContextPolicy::new(12, 2, 24).unwrap(),
    })
    .unwrap();
    assert!(project.can_transition(&todo, &doing));
    assert!(!project.can_transition(&todo, &done));
}
