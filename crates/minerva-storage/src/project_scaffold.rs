use minerva_domain::{
    ContextPolicy, Project, ProjectConfig, ProjectId, StatusDefinition, StatusKey,
    StatusTransition, TaskTypeKey,
};
use std::{path::Path, time::SystemTime};

pub const SCHEMA_VERSION: &str = "1\n";
const FEATURE_TASK_TYPE: &str = "---\nname: feature\ndisplay_name: Feature\ndescription: User-facing capability or workflow change.\n---\n# Feature\n\nDescribe the user-facing capability.\n";
const BUG_TASK_TYPE: &str = "---\nname: bug\ndisplay_name: Bug\ndescription: Incorrect behavior that should be fixed.\n---\n# Bug\n\nDescribe the defect and expected behavior.\n";
const RESEARCH_TASK_TYPE: &str = "---\nname: research\ndisplay_name: Research\ndescription: Investigation that produces findings or recommendations.\n---\n# Research\n\nCapture the question and findings.\n";
const REFACTOR_TASK_TYPE: &str = "---\nname: refactor\ndisplay_name: Refactor\ndescription: Internal code change that preserves behavior.\n---\n# Refactor\n\nDescribe the code change and constraints.\n";
const DOCUMENTATION_TASK_TYPE: &str = "---\nname: documentation\ndisplay_name: Documentation\ndescription: Docs-only change for users or maintainers.\n---\n# Documentation\n\nDescribe the docs update.\n";
const CHORE_TASK_TYPE: &str = "---\nname: chore\ndisplay_name: Chore\ndescription: Maintenance work with limited product impact.\n---\n# Chore\n\nDescribe the maintenance task.\n";

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
