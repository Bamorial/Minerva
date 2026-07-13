use crate::{LayoutClass, LayoutEntry};

const PROJECT_LAYOUT: [LayoutEntry; 10] = [
    LayoutEntry::new("project.yaml", LayoutClass::Canonical, "project metadata"),
    LayoutEntry::new("config.yaml", LayoutClass::Canonical, "user configuration"),
    LayoutEntry::new("instructions.md", LayoutClass::Canonical, "project instructions"),
    LayoutEntry::new("schema-version", LayoutClass::Canonical, "schema marker"),
    LayoutEntry::new("task-types/", LayoutClass::Canonical, "task type templates"),
    LayoutEntry::new("tasks/", LayoutClass::Canonical, "canonical task records"),
    LayoutEntry::new("indexes/", LayoutClass::Derived, "rebuildable indexes"),
    LayoutEntry::new("contexts/", LayoutClass::Derived, "compiled contexts"),
    LayoutEntry::new("sessions/", LayoutClass::Derived, "execution manifests"),
    LayoutEntry::new("locks/", LayoutClass::Operational, "runtime lock files"),
];

const TASK_LAYOUT: [LayoutEntry; 5] = [
    LayoutEntry::new(
        "tasks/<task-id>/task.yaml",
        LayoutClass::Canonical,
        "task metadata",
    ),
    LayoutEntry::new(
        "tasks/<task-id>/instructions.md",
        LayoutClass::Canonical,
        "task instructions",
    ),
    LayoutEntry::new(
        "tasks/<task-id>/declaration.md",
        LayoutClass::Canonical,
        "task handoff state",
    ),
    LayoutEntry::new("tasks/<task-id>/notes.md", LayoutClass::Canonical, "task notes"),
    LayoutEntry::new(
        "tasks/<task-id>/events.jsonl",
        LayoutClass::Canonical,
        "task audit log",
    ),
];

pub const fn project_layout() -> &'static [LayoutEntry] {
    &PROJECT_LAYOUT
}

pub const fn task_layout() -> &'static [LayoutEntry] {
    &TASK_LAYOUT
}
