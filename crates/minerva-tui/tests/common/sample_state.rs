#![allow(dead_code)]

use minerva_application::{
    TaskShowFreshness, TaskShowLink, TaskShowRelationship, TaskShowResult,
    TaskShowTimestamps, TaskTreeNode,
};
use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationMetadata, StatusKey, Task, TaskFacts,
    TaskId, TaskIdAllocator, TaskPriority, TaskResources, TaskTypeKey, TaskVersion,
};
use minerva_tui::AppState;
use std::{collections::BTreeSet, path::PathBuf, time::UNIX_EPOCH};

pub fn sample_state(title: &str) -> AppState {
    let mut state = AppState::new(PathBuf::from("."));
    let mut task = sample_task(title, None, false);
    task.facts.modules = vec!["minerva-tui".into()];
    task.facts.files = vec!["crates/minerva-tui/src/render_task_detail.rs".into()];
    task.facts.acceptance_checks = vec!["cargo test -p minerva-tui".into()];
    task.facts.resources = TaskResources {
        reads: vec![".tasker/current/WORKSPACE.md".into()],
        writes: vec!["declaration.md".into()],
    };
    state.set_tree(vec![TaskTreeNode { task: task.clone(), children: Vec::new() }]);
    state.detail = Some(sample_detail(task));
    state
}

pub fn sample_task(title: &str, parent_id: Option<TaskId>, archived: bool) -> Task {
    Task::new(Task {
        schema_version: 1,
        id: TaskIdAllocator::new(title.len() as u32).next_id(),
        title: title.into(),
        slug: None,
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: StatusKey::new("backlog").unwrap(),
        parent_id,
        priority: TaskPriority::Medium,
        tags: BTreeSet::new(),
        created_at: UNIX_EPOCH,
        updated_at: UNIX_EPOCH,
        completed_at: None,
        version: TaskVersion::initial(),
        declaration: DeclarationMetadata {
            version: 1,
            updated_at: UNIX_EPOCH,
            updated_by: DeclarationActor::Human,
            commit_hash: None,
        },
        facts: TaskFacts::default(),
        archive_state: if archived {
            ArchiveState::Archived
        } else {
            ArchiveState::Active
        },
    })
    .unwrap()
}

fn sample_detail(task: Task) -> TaskShowResult {
    TaskShowResult {
        task,
        parent: None,
        dependencies: Vec::new(),
        relationships: vec![TaskShowRelationship {
            kind: "related-to".into(),
            direction: "outgoing".into(),
            task: TaskShowLink { id: "TSK-0002".into(), title: "Other task".into() },
            reason: Some("shared detail pane".into()),
            created_at: "1970-01-01T00:00:00Z".into(),
        }],
        freshness: TaskShowFreshness {
            status: "stale".into(),
            reasons: vec!["covered-commit-differs".into()],
        },
        timestamps: TaskShowTimestamps {
            created_at: "1970-01-01T00:00:00Z".into(),
            updated_at: "1970-01-01T00:00:00Z".into(),
            completed_at: None,
            declaration_updated_at: "1970-01-01T00:00:00Z".into(),
        },
        instructions: None,
        declaration: Some(sample_declaration().into()),
    }
}

fn sample_declaration() -> &'static str {
    "# Declaration\n\n## Objective\nShip the detail pane.\n\n## Current State\n- Detail pane is implemented.\n- Declaration summary is visible.\n\n## Completed Work\nRenderer and scrolling are wired.\n\n## Remaining Work\nRun tests.\n\n## Decisions\nUse deterministic summaries.\n\n## Risks\nNone.\n\n## Verification\nPending.\n\n## Open Questions\nNone."
}
