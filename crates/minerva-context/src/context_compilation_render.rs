use crate::{
    ContextInclusionReason, ContextSelectionItem, render_target_metadata,
    render_task_summary,
};
use minerva_domain::ContextDetail;
use minerva_domain::DeclarationDocument;
use minerva_domain::RelationshipType;

const PROJECT_INSTRUCTIONS_PLACEHOLDER: &str =
    "# Project Instructions\n\nAdd repository-wide Minerva instructions here.";

pub fn detail_text(text: &str, detail: ContextDetail) -> String {
    match detail {
        ContextDetail::Full => text.trim().to_owned(),
        ContextDetail::Summary => text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .take(8)
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn declaration_text(text: &str, detail: ContextDetail) -> Option<String> {
    (!DeclarationDocument::is_effectively_empty(text))
        .then(|| detail_text(text, detail))
}

pub fn project_instructions_text(text: &str, detail: ContextDetail) -> Option<String> {
    let trimmed = text.trim();
    (!trimmed.is_empty() && trimmed != PROJECT_INSTRUCTIONS_PLACEHOLDER)
        .then(|| detail_text(text, detail))
}

pub fn render_collection(items: &[(&ContextSelectionItem, String)]) -> String {
    items.iter().map(|(item, body)| block(item, body)).collect::<Vec<_>>().join("\n\n")
}

pub fn render_related(items: &[ContextSelectionItem], full: bool) -> String {
    items
        .iter()
        .map(|item| {
            let body = if full {
                render_target_metadata(&item.task)
            } else {
                render_task_summary(&item.task)
            };
            block(item, &body)
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn block(item: &ContextSelectionItem, body: &str) -> String {
    format!(
        "### {} {}\nincluded_because: {}\n\n{}",
        item.task.id,
        item.task.title,
        reason(item.reason),
        body.trim()
    )
}

fn reason(value: ContextInclusionReason) -> String {
    match value {
        ContextInclusionReason::Target => "target".into(),
        ContextInclusionReason::Ancestor { depth } => format!("ancestor depth {depth}"),
        ContextInclusionReason::Dependency { depth, relationship_type } => {
            format!("dependency depth {depth} via {}", relationship(relationship_type))
        }
        ContextInclusionReason::RelatedTask { depth, relationship_type, direction } => {
            format!(
                "related depth {depth} {direction:?} via {}",
                relationship(relationship_type)
            )
        }
        ContextInclusionReason::Child { depth } => format!("child depth {depth}"),
        ContextInclusionReason::Sibling { depth } => format!("sibling depth {depth}"),
    }
}

fn relationship(value: RelationshipType) -> &'static str {
    match value {
        RelationshipType::Parent => "parent",
        RelationshipType::DependsOn => "depends-on",
        RelationshipType::Blocks => "blocks",
        RelationshipType::RelatedTo => "related-to",
        RelationshipType::Duplicates => "duplicates",
        RelationshipType::Implements => "implements",
        RelationshipType::References => "references",
    }
}
