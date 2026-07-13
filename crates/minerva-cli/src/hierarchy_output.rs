use crate::response::CommandOutput;
use minerva_application::TaskHierarchyQueryResult;
use minerva_domain::Task;
use serde_json::json;

pub fn children(result: &TaskHierarchyQueryResult) -> CommandOutput {
    let summary = format!("{} has {} children", result.task.id, result.items.len());
    CommandOutput::with_json(lines(summary, &result.items), body("children", result))
}

pub fn ancestors(result: &TaskHierarchyQueryResult) -> CommandOutput {
    let summary = format!("{} has {} ancestors", result.task.id, result.items.len());
    CommandOutput::with_json(lines(summary, &result.items), body("ancestors", result))
}

fn lines(summary: String, items: &[Task]) -> String {
    let mut lines = vec![summary];
    lines.extend(items.iter().map(line));
    lines.join("\n")
}

fn line(task: &Task) -> String {
    format!("{} [{}] {}", task.id, task.status, task.title)
}

fn body(kind: &str, result: &TaskHierarchyQueryResult) -> serde_json::Value {
    json!({
        "kind": kind,
        "task": item(&result.task),
        "items": result.items.iter().map(item).collect::<Vec<_>>(),
    })
}

fn item(task: &Task) -> serde_json::Value {
    json!({
        "id": task.id,
        "title": task.title,
        "status": task.status,
        "type": task.task_type,
        "parent_id": task.parent_id,
    })
}
