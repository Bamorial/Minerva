use crate::response::CommandOutput;
use minerva_domain::Relationship;
use serde_json::json;

pub fn created(relationship: Relationship) -> CommandOutput {
    render("created", relationship)
}

pub fn removed(relationship: Relationship) -> CommandOutput {
    render("removed", relationship)
}

fn render(action: &str, relationship: Relationship) -> CommandOutput {
    CommandOutput::with_json(
        format!(
            "{action} {} {} -> {} ({})",
            kind(&relationship),
            relationship.source_task,
            relationship.target_task,
            relationship.id
        ),
        json!({
            "action": action,
            "relationship": {
                "id": relationship.id,
                "source_task": relationship.source_task,
                "target_task": relationship.target_task,
                "type": kind(&relationship),
                "reason": relationship.reason,
            }
        }),
    )
}

fn kind(relationship: &Relationship) -> &'static str {
    match relationship.relationship_type {
        minerva_domain::RelationshipType::Parent => "parent",
        minerva_domain::RelationshipType::DependsOn => "depends_on",
        minerva_domain::RelationshipType::Blocks => "blocks",
        minerva_domain::RelationshipType::RelatedTo => "related_to",
        minerva_domain::RelationshipType::Duplicates => "duplicates",
        minerva_domain::RelationshipType::Implements => "implements",
        minerva_domain::RelationshipType::References => "references",
    }
}
