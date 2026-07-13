use crate::response::CommandOutput;
use minerva_application::{TaskLogEvent, TaskLogIssue, TaskLogResult};
use std::fmt::Write;

pub fn render(result: &TaskLogResult) -> CommandOutput {
    CommandOutput::with_json(text(result), json(result))
}

fn text(result: &TaskLogResult) -> String {
    let mut text = format!("{} {} event(s)", result.task.id, result.events.len());
    if !result.filters.is_empty() {
        let kinds = result
            .filters
            .iter()
            .map(|kind| kind.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let _ = write!(text, " filtered by {kinds}");
    }
    if result.events.is_empty() {
        text.push_str("\nno events recorded");
    }
    for event in &result.events {
        let _ = write!(
            text,
            "\n{} | {} | {}",
            event.recorded_at,
            event.actor,
            event.kind.as_str(),
        );
        if !event.details.is_empty() {
            let _ = write!(text, " | {}", event.details.join(" "));
        }
    }
    for issue in &result.issues {
        let _ = write!(text, "\nmalformed line {}: {}", issue.line, issue.reason);
    }
    text
}

fn json(result: &TaskLogResult) -> serde_json::Value {
    serde_json::json!({
        "task": {
            "id": result.task.id,
            "title": result.task.title,
        },
        "filters": result.filters.iter().map(|kind| kind.as_str()).collect::<Vec<_>>(),
        "events": result.events.iter().map(event).collect::<Vec<_>>(),
        "issues": result.issues.iter().map(issue).collect::<Vec<_>>(),
    })
}

fn event(event: &TaskLogEvent) -> serde_json::Value {
    serde_json::json!({
        "id": event.id,
        "recorded_at": event.recorded_at,
        "actor": event.actor,
        "kind": event.kind.as_str(),
        "details": event.details,
    })
}

fn issue(issue: &TaskLogIssue) -> serde_json::Value {
    serde_json::json!({
        "line": issue.line,
        "reason": issue.reason,
    })
}
