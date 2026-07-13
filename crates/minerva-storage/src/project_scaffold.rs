use minerva_domain::{
    ContextPolicy, Project, ProjectConfig, ProjectId, StatusDefinition, StatusKey,
    StatusTransition, TaskTypeKey,
};
use std::{path::Path, time::SystemTime};

pub const SCHEMA_VERSION: &str = "1\n";
pub const TASK_TYPES: [(&str, &str); 6] = [
    ("feature.md", "# Feature\n\nDescribe the user-facing capability.\n"),
    ("bug.md", "# Bug\n\nDescribe the defect and expected behavior.\n"),
    ("research.md", "# Research\n\nCapture the question and findings.\n"),
    ("refactor.md", "# Refactor\n\nDescribe the code change and constraints.\n"),
    ("documentation.md", "# Documentation\n\nDescribe the docs update.\n"),
    ("chore.md", "# Chore\n\nDescribe the maintenance task.\n"),
];

pub fn default_project(root: &Path) -> Project {
    let backlog = status("backlog");
    let in_progress = status("in-progress");
    let completed = status("completed");
    Project::new(Project {
        schema_version: 1,
        id: ProjectId::new(),
        name: project_name(root),
        created_at: SystemTime::now(),
        default_task_type: task_type("feature"),
        default_status: backlog.clone(),
        statuses: vec![
            StatusDefinition::new(backlog.clone(), false),
            StatusDefinition::new(in_progress.clone(), false),
            StatusDefinition::new(completed.clone(), true),
        ],
        transitions: vec![
            StatusTransition::new(backlog, in_progress.clone()),
            StatusTransition::new(in_progress, completed),
        ],
        context_policy: ContextPolicy::new(12, 2, 24).expect("static policy is valid"),
    })
    .expect("default project is valid")
}

pub fn default_config() -> ProjectConfig {
    ProjectConfig::new(ProjectConfig { schema_version: 1, editor: None })
        .expect("default config is valid")
}

pub const fn agents_md() -> &'static str {
    "# Minerva Project\n\nThis repository uses Minerva for task management.\n"
}

pub const fn instructions_md() -> &'static str {
    "# Project Instructions\n\nAdd repository-wide Minerva instructions here.\n"
}

fn project_name(root: &Path) -> String {
    root.file_name().and_then(|value| value.to_str()).unwrap_or("Minerva").into()
}

fn status(value: &str) -> StatusKey {
    StatusKey::new(value).expect("static status is valid")
}

fn task_type(value: &str) -> TaskTypeKey {
    TaskTypeKey::new(value).expect("static task type is valid")
}
