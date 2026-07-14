use crate::{
    task_detail_declaration::declaration_summary,
    task_detail_format::{
        label, link, list, list_or_message, priority, section, yes_no,
    },
};
use minerva_application::{TaskShowLink, TaskShowRelationship, TaskShowResult};
use ratatui::text::Line;

pub fn build_lines(detail: &TaskShowResult) -> Vec<Line<'static>> {
    let mut lines = identity(detail);
    lines.extend(link_section("Dependencies", &detail.dependencies));
    lines.extend(relationship_section(&detail.relationships));
    lines.extend(facts(detail));
    lines.extend(freshness(detail));
    lines.extend(section(
        "Declaration Summary",
        declaration_summary(detail.declaration.as_deref()),
    ));
    lines
}

fn identity(detail: &TaskShowResult) -> Vec<Line<'static>> {
    vec![
        Line::from(detail.task.title.clone()),
        Line::from(format!(
            "{}  {}  {}",
            detail.task.id, detail.task.status, detail.task.task_type
        )),
        Line::from(format!("Priority: {}", priority(detail.task.priority))),
        Line::from(format!("Parent: {}", link(detail.parent.as_ref()))),
        Line::from(String::new()),
    ]
}

fn relationship_section(values: &[TaskShowRelationship]) -> Vec<Line<'static>> {
    let body = values
        .iter()
        .map(|value| {
            let reason = value.reason.as_deref().unwrap_or(&value.created_at);
            format!(
                "{} {} {} ({reason})",
                value.kind,
                value.direction,
                label(&value.task)
            )
        })
        .collect();
    section("Relationships", list_or_message(body, "No relationships."))
}

fn freshness(detail: &TaskShowResult) -> Vec<Line<'static>> {
    let mut body = vec![format!("Status: {}", detail.freshness.status)];
    body.extend(list_or_message(
        detail.freshness.reasons.clone(),
        "No freshness warnings.",
    ));
    section("Declaration Freshness", body)
}

fn facts(detail: &TaskShowResult) -> Vec<Line<'static>> {
    let facts = &detail.task.facts;
    section(
        "Facts",
        vec![
            list("Modules", &facts.modules),
            list("Files", &facts.files),
            format!("Migrations Required: {}", yes_no(facts.migrations_required)),
            list("Feature Flags", &facts.feature_flags),
            list("Acceptance Checks", &facts.acceptance_checks),
            list("Reads", &facts.resources.reads),
            list("Writes", &facts.resources.writes),
        ],
    )
}

fn link_section(title: &str, values: &[TaskShowLink]) -> Vec<Line<'static>> {
    section(
        title,
        list_or_message(
            values.iter().map(label).collect(),
            &format!("No {}.", title.to_lowercase()),
        ),
    )
}
