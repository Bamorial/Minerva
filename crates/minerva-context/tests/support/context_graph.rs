#![allow(dead_code)]

use crate::support::{
    persist_task, refresh_declaration, relate, repo, write_project_instructions,
};
use minerva_domain::{
    ContextDetail, ContextPolicy, ContextRelationPolicy, RelationshipType, Task,
};
use std::path::PathBuf;

pub fn realistic_graph() -> (PathBuf, Task, Task, ContextPolicy) {
    let root = repo("compile");
    write_project_instructions(&root, "# Project\n\nKeep context deterministic.");
    let parent = persist_task(
        &root,
        1,
        "Parent",
        None,
        "# Parent\n\nGuard invariants.",
        "# Declaration\n\nParent context.",
        &["parent check"],
    );
    let target = persist_task(
        &root,
        2,
        "Target",
        Some(parent.id),
        "# Target\n\nImplement compiler.",
        "# Declaration\n\nTarget state.",
        &["context compiles", "manifest renders"],
    );
    let dependency = persist_task(
        &root,
        3,
        "Dependency",
        None,
        "# Dependency\n\nNeeded first.",
        "# Declaration\n\nDependency state.",
        &["dependency ready"],
    );
    let related = persist_task(
        &root,
        4,
        "Related",
        None,
        "# Related\n\nCoordinate output.",
        "# Declaration\n\nRelated state.",
        &["related understood"],
    );
    let child = persist_task(
        &root,
        5,
        "Child",
        Some(target.id),
        "# Child\n\nFollow up work.",
        "# Declaration\n\nChild state.",
        &["child checked"],
    );
    relate(&root, target.id, dependency.id, RelationshipType::DependsOn);
    relate(&root, target.id, related.id, RelationshipType::References);
    refresh_declaration(&root, target.id, "Target state.");
    refresh_declaration(&root, dependency.id, "Dependency state.");
    (root, target, child, policy())
}

fn policy() -> ContextPolicy {
    ContextPolicy {
        related_tasks: Some(ContextRelationPolicy {
            detail: ContextDetail::Summary,
            depth: 1,
        }),
        children: Some(ContextRelationPolicy {
            detail: ContextDetail::Summary,
            depth: 1,
        }),
        ..ContextPolicy::strict()
    }
}
