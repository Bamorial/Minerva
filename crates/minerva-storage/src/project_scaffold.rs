use minerva_domain::{
    ContextPolicy, Project, ProjectConfig, ProjectId, StatusDefinition, StatusKey,
    StatusTransition, TaskPriority, TaskTypeKey,
};
use std::collections::BTreeSet;
use std::{path::Path, time::SystemTime};

pub const SCHEMA_VERSION: &str = "1\n";
const FEATURE_TASK_TYPE: &str = include_str!("task_type_templates/feature.md");
const BUG_TASK_TYPE: &str = include_str!("task_type_templates/bug.md");
const RESEARCH_TASK_TYPE: &str = include_str!("task_type_templates/research.md");
const REFACTOR_TASK_TYPE: &str = include_str!("task_type_templates/refactor.md");
const DOCUMENTATION_TASK_TYPE: &str =
    include_str!("task_type_templates/documentation.md");
const CHORE_TASK_TYPE: &str = include_str!("task_type_templates/chore.md");

pub const TASK_TYPES: [(&str, &str); 6] = [
    ("feature.md", FEATURE_TASK_TYPE),
    ("bug.md", BUG_TASK_TYPE),
    ("research.md", RESEARCH_TASK_TYPE),
    ("refactor.md", REFACTOR_TASK_TYPE),
    ("documentation.md", DOCUMENTATION_TASK_TYPE),
    ("chore.md", CHORE_TASK_TYPE),
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
            StatusTransition::new(in_progress.clone(), completed.clone()),
            StatusTransition::new(completed, in_progress),
        ],
        context_policy: ContextPolicy::new(12, 2, 24).expect("static policy is valid"),
    })
    .expect("default project is valid")
}

pub fn default_config() -> ProjectConfig {
    ProjectConfig::new(ProjectConfig {
        schema_version: 1,
        editor: None,
        default_priority: TaskPriority::Medium,
        default_tags: BTreeSet::new(),
    })
    .expect("default config is valid")
}

pub const fn agents_md() -> &'static str {
    "# Minerva Project\n\n\
This repository uses Minerva for task and context management.\n\n\
Before starting work:\n\n\
1. Read `.minerva/instructions.md` for project-specific rules.\n\
2. Read the current task's `task.md`, `instructions.md`, and `declaration.md`.\n\
3. Prefer Minerva CLI or MCP operations over manual edits to task metadata.\n\
4. Update `declaration.md` after meaningful progress, decisions, or blockers.\n\
5. Validate task state before marking work complete.\n\n\
Preferred operations:\n\n\
- Create and update tasks through Minerva tools.\n\
- Change status and dependencies through Minerva tools.\n\
- Keep detailed project guidance in `.minerva/instructions.md`.\n"
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
