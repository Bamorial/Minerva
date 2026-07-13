use crate::response::CommandOutput;
use minerva_application::{TaskShowLink, TaskShowRelationship, TaskShowResult};
use serde_json::json;

pub fn render(result: &TaskShowResult) -> CommandOutput {
    let text = result.render();
    let json = json!({
        "task": {
            "id": result.task.id,
            "title": result.task.title,
            "type": result.task.task_type,
            "status": result.task.status,
            "priority": priority(result.task.priority),
            "version": result.task.version.get(),
            "declaration_version": result.task.declaration.version,
            "parent": result.parent.as_ref().map(link),
            "dependencies": result.dependencies.iter().map(link).collect::<Vec<_>>(),
            "relationships": result.relationships.iter().map(relationship).collect::<Vec<_>>(),
            "declaration_freshness": {
                "status": result.freshness.status,
                "reasons": result.freshness.reasons,
            },
            "facts": result.task.facts,
            "timestamps": {
                "created_at": result.timestamps.created_at,
                "updated_at": result.timestamps.updated_at,
                "completed_at": result.timestamps.completed_at,
                "declaration_updated_at": result.timestamps.declaration_updated_at,
            },
            "instructions": result.instructions,
            "declaration": result.declaration,
        }
    });
    CommandOutput::with_json(text, json)
}

fn link(value: &TaskShowLink) -> serde_json::Value {
    json!({ "id": value.id, "title": value.title })
}

fn relationship(value: &TaskShowRelationship) -> serde_json::Value {
    json!({
        "kind": value.kind,
        "direction": value.direction,
        "task": link(&value.task),
        "reason": value.reason,
        "created_at": value.created_at,
    })
}

fn priority(value: minerva_domain::TaskPriority) -> &'static str {
    match value {
        minerva_domain::TaskPriority::Low => "low",
        minerva_domain::TaskPriority::Medium => "medium",
        minerva_domain::TaskPriority::High => "high",
        minerva_domain::TaskPriority::Urgent => "urgent",
    }
}
