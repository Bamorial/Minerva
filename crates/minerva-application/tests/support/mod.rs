#![allow(dead_code, unused_imports)]

mod repos;

pub use repos::{FakeProjectRepo, FakeTaskRepo, task};

use minerva_domain::{
    ContextPolicy, Project, ProjectConfig, ProjectId, StatusDefinition, StatusKey,
    StatusTransition, TaskPriority, TaskTypeDefinition, TaskTypeKey,
};
use std::collections::BTreeSet;
use std::time::UNIX_EPOCH;

pub fn project() -> Project {
    Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: "Minerva".into(),
        created_at: UNIX_EPOCH,
        default_task_type: TaskTypeKey::new("feature").unwrap(),
        default_status: StatusKey::new("backlog").unwrap(),
        statuses: vec![
            StatusDefinition::new(StatusKey::new("backlog").unwrap(), false),
            StatusDefinition::new(StatusKey::new("completed").unwrap(), true),
        ],
        transitions: vec![StatusTransition::new(
            StatusKey::new("backlog").unwrap(),
            StatusKey::new("completed").unwrap(),
        )],
        context_policy: ContextPolicy::new(12, 2, 24).unwrap(),
    })
    .unwrap()
}

pub fn config() -> ProjectConfig {
    ProjectConfig::new(ProjectConfig {
        schema_version: 1,
        editor: None,
        default_priority: TaskPriority::High,
        default_tags: BTreeSet::from(
            [minerva_domain::TaskTag::new("release").unwrap()],
        ),
    })
    .unwrap()
}

pub fn task_type(name: &str, template: &str) -> TaskTypeDefinition {
    TaskTypeDefinition::new(TaskTypeDefinition {
        name: TaskTypeKey::new(name).unwrap(),
        display_name: name.into(),
        description: None,
        declaration_requirements: Vec::new(),
        instruction_template: template.into(),
    })
    .unwrap()
}
