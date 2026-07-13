use crate::{TaskShowLink, TaskShowRelationship, TaskShowResult, render_task_facts};

impl TaskShowResult {
    #[must_use]
    pub fn render(&self) -> String {
        let mut lines = vec![
            format!("{} {}", self.task.id, self.task.title),
            format!("type: {}", self.task.task_type),
            format!("status: {}", self.task.status),
            format!("priority: {}", priority(self.task.priority)),
            format!("parent: {}", item(self.parent.as_ref())),
            format!("dependencies: {}", items(&self.dependencies)),
            format!("declaration freshness: {}", self.freshness.status),
            format!("freshness reasons: {}", strings(&self.freshness.reasons)),
            format!("created_at: {}", self.timestamps.created_at),
            format!("updated_at: {}", self.timestamps.updated_at),
            format!(
                "completed_at: {}",
                option(self.timestamps.completed_at.as_deref())
            ),
            format!(
                "declaration_updated_at: {}",
                self.timestamps.declaration_updated_at
            ),
            format!("version: {}", self.task.version.get()),
            format!("declaration version: {}", self.task.declaration.version),
            render_task_facts(&self.task),
        ];
        lines.extend(render_relationships(&self.relationships));
        append_section(&mut lines, "instructions", self.instructions.as_deref());
        append_section(&mut lines, "declaration", self.declaration.as_deref());
        lines.join("\n")
    }
}

fn render_relationships(values: &[TaskShowRelationship]) -> Vec<String> {
    if values.is_empty() {
        return vec!["relationships: none".into()];
    }
    std::iter::once("relationships:".into())
        .chain(values.iter().map(|value| {
            format!(
                "- {} {} {} ({})",
                value.kind,
                value.direction,
                label(&value.task),
                value.reason.clone().unwrap_or_else(|| value.created_at.clone())
            )
        }))
        .collect()
}

fn append_section(lines: &mut Vec<String>, title: &str, body: Option<&str>) {
    if let Some(body) = body {
        lines.push(String::new());
        lines.push(format!("{title}:"));
        lines.push(body.trim_end().into());
    }
}

fn items(values: &[TaskShowLink]) -> String {
    if values.is_empty() {
        "none".into()
    } else {
        values.iter().map(label).collect::<Vec<_>>().join(", ")
    }
}

fn item(value: Option<&TaskShowLink>) -> String {
    value.map_or_else(|| "none".into(), label)
}

fn label(value: &TaskShowLink) -> String {
    format!("{} {}", value.id, value.title)
}

fn option(value: Option<&str>) -> String {
    value.unwrap_or("none").into()
}

fn strings(values: &[String]) -> String {
    if values.is_empty() { "none".into() } else { values.join(", ") }
}

fn priority(value: minerva_domain::TaskPriority) -> &'static str {
    match value {
        minerva_domain::TaskPriority::Low => "low",
        minerva_domain::TaskPriority::Medium => "medium",
        minerva_domain::TaskPriority::High => "high",
        minerva_domain::TaskPriority::Urgent => "urgent",
    }
}
