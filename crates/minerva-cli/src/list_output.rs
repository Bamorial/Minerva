use crate::response::CommandOutput;
use minerva_application::{TaskListItem, TaskListResult, TaskListSort};
use minerva_domain::{ArchiveState, TaskPriority};
use serde_json::json;
use std::fmt::Write;

pub fn render(result: &TaskListResult) -> CommandOutput {
    CommandOutput::with_json(text(result), json(result))
}

fn text(result: &TaskListResult) -> String {
    let mut text = format!(
        "showing {} of {} matching tasks (total {}, sort {}, offset {}, limit {})",
        result.tasks.len(),
        result.matched,
        result.total,
        sort(result.sort),
        result.offset,
        result.limit.map_or_else(|| "all".into(), |value| value.to_string()),
    );
    for task in &result.tasks {
        let _ = writeln!(text);
        let _ = write!(
            text,
            "{} {} | {} | {} | {} | {}",
            task.task.id,
            task.task.title,
            task.task.status,
            task.task.task_type,
            priority(task.task.priority),
            archive(task.task.archive_state),
        );
        if let Some(parent) = &task.parent {
            let _ = write!(text, " | parent={}", parent.id);
        }
        if !task.task.tags.is_empty() {
            let _ = write!(text, " | tags={}", tags(task));
        }
    }
    if result.tasks.is_empty() {
        text.push_str("\nno tasks matched");
    } else if result.has_more {
        text.push_str("\nmore tasks available; use --offset or --all");
    }
    text
}

fn json(result: &TaskListResult) -> serde_json::Value {
    json!({
        "tasks": result.tasks.iter().map(task).collect::<Vec<_>>(),
        "total": result.total, "matched": result.matched, "offset": result.offset,
        "limit": result.limit, "sort": sort(result.sort), "has_more": result.has_more,
    })
}

fn task(value: &TaskListItem) -> serde_json::Value {
    json!({
        "id": value.task.id.to_string(), "title": &value.task.title,
        "type": value.task.task_type.as_str(), "status": value.task.status.as_str(),
        "priority": priority(value.task.priority),
        "archive_state": archive(value.task.archive_state),
        "parent": value.parent.as_ref().map(|parent| json!({"id": &parent.id, "title": &parent.title})),
        "tags": value.task.tags.iter().map(minerva_domain::TaskTag::as_str).collect::<Vec<_>>(),
        "timestamps": {
            "created_at": humantime::format_rfc3339(value.task.created_at).to_string(),
            "updated_at": humantime::format_rfc3339(value.task.updated_at).to_string(),
            "completed_at": value.task.completed_at.map(|time| humantime::format_rfc3339(time).to_string()),
        },
    })
}

fn tags(value: &TaskListItem) -> String {
    value
        .task
        .tags
        .iter()
        .map(minerva_domain::TaskTag::as_str)
        .collect::<Vec<_>>()
        .join(",")
}
const fn sort(value: TaskListSort) -> &'static str {
    match value {
        TaskListSort::Created => "created",
        TaskListSort::Updated => "updated",
        TaskListSort::Priority => "priority",
        TaskListSort::Title => "title",
        TaskListSort::Id => "id",
    }
}
const fn priority(value: TaskPriority) -> &'static str {
    match value {
        TaskPriority::Low => "low",
        TaskPriority::Medium => "medium",
        TaskPriority::High => "high",
        TaskPriority::Urgent => "urgent",
    }
}
const fn archive(value: ArchiveState) -> &'static str {
    match value {
        ArchiveState::Active => "active",
        ArchiveState::Archived => "archived",
    }
}
