use crate::response::CommandOutput;
use minerva_application::{TaskTreeNode, TaskTreeResult};
use minerva_domain::ArchiveState;
use serde_json::json;

pub fn render(result: TaskTreeResult) -> CommandOutput {
    CommandOutput::with_json(text(&result), json(&result))
}

fn text(result: &TaskTreeResult) -> String {
    let mut lines = vec![format!(
        "showing {} matching tasks across {} roots (total {})",
        result.matched,
        result.roots.len(),
        result.total,
    )];
    for (index, node) in result.roots.iter().enumerate() {
        render_node(node, "", index + 1 == result.roots.len(), &mut lines);
    }
    if result.roots.is_empty() {
        lines.push("no tasks matched".into());
    }
    lines.join("\n")
}

fn json(result: &TaskTreeResult) -> serde_json::Value {
    json!({
        "roots": result.roots.iter().map(node).collect::<Vec<_>>(),
        "total": result.total,
        "matched": result.matched,
    })
}

fn node(value: &TaskTreeNode) -> serde_json::Value {
    json!({
        "id": value.task.id.to_string(),
        "title": &value.task.title,
        "type": value.task.task_type.as_str(),
        "status": value.task.status.as_str(),
        "archive_state": archive(value.task.archive_state),
        "children": value.children.iter().map(node).collect::<Vec<_>>(),
    })
}

fn render_node(node: &TaskTreeNode, prefix: &str, last: bool, lines: &mut Vec<String>) {
    let branch = if last { "`--" } else { "|--" };
    lines.push(format!(
        "{prefix}{branch} {} [{}] {}",
        node.task.id, node.task.status, node.task.title
    ));
    let next = format!("{prefix}{}", if last { "   " } else { "|  " });
    for (index, child) in node.children.iter().enumerate() {
        render_node(child, &next, index + 1 == node.children.len(), lines);
    }
}

const fn archive(value: ArchiveState) -> &'static str {
    match value {
        ArchiveState::Active => "active",
        ArchiveState::Archived => "archived",
    }
}
