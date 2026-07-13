use crate::cli::{ContextArgs, ContextFormatArg};
use minerva_context::{
    ContextCompilationResult, ContextInclusionReason, ContextRelationshipDirection,
};
use serde_json::{Value, json};

pub struct RenderedContext {
    pub text: String,
    pub json: Value,
}

pub fn render(
    args: &ContextArgs,
    result: &ContextCompilationResult,
) -> Result<RenderedContext, minerva_domain::MinervaError> {
    let json = json_value(args, result);
    let text = match args.format {
        ContextFormatArg::Markdown => markdown(args, result),
        ContextFormatArg::Json => {
            serde_json::to_string_pretty(&json).map_err(schema)?
        }
    };
    Ok(RenderedContext { text, json })
}

fn markdown(args: &ContextArgs, result: &ContextCompilationResult) -> String {
    (!args.explain).then(|| result.markdown.clone()).unwrap_or_else(|| {
        format!("{}\n\n## Context Explain\n\n{}", result.markdown, explain(result))
    })
}

fn json_value(args: &ContextArgs, result: &ContextCompilationResult) -> Value {
    let explain = args.explain.then(|| {
        json!({
            "included_tasks": result.selection.items.iter().map(|item| json!({
                "task": &item.task, "reason": reason_json(item.reason),
            })).collect::<Vec<_>>(),
            "excluded_sections": result.excluded_sections.iter().map(|section| json!({
                "source": section.id().source_key(),
                "reason": section.reason().as_str(),
                "estimated_tokens": section.estimated_tokens(),
                "input_hash": section.input_hash(),
            })).collect::<Vec<_>>(),
        })
    });
    json!({
        "task_ref": &args.task_ref,
        "format": format_name(args.format),
        "budget": args.budget,
        "estimated_tokens": result.estimated_tokens,
        "content": &result.markdown,
        "manifest": &result.manifest,
        "explain": explain,
    })
}

fn explain(result: &ContextCompilationResult) -> String {
    let mut lines = result.selection.items.iter().map(included).collect::<Vec<_>>();
    if result.excluded_sections.is_empty() {
        lines.push("excluded_sections: none".into());
    } else {
        lines.extend(result.excluded_sections.iter().map(excluded));
    }
    lines.join("\n")
}

fn included(item: &minerva_context::ContextSelectionItem) -> String {
    format!("included: {} {}", item.task.id, reason_text(item.reason))
}

fn excluded(section: &minerva_context::ContextSectionExclusion) -> String {
    format!(
        "excluded: {} {} {}",
        section.id().source_key(),
        section.reason().as_str(),
        section.estimated_tokens()
    )
}

fn reason_text(value: ContextInclusionReason) -> String {
    match value {
        ContextInclusionReason::Target => "target".into(),
        ContextInclusionReason::Ancestor { depth } => format!("ancestor depth {depth}"),
        ContextInclusionReason::Child { depth } => format!("child depth {depth}"),
        ContextInclusionReason::Sibling { depth } => format!("sibling depth {depth}"),
        ContextInclusionReason::Dependency { depth, relationship_type } => {
            format!("dependency {} depth {depth}", relationship_name(relationship_type))
        }
        ContextInclusionReason::RelatedTask { depth, relationship_type, direction } => {
            format!(
                "related {} {} depth {depth}",
                relationship_name(relationship_type),
                direction_name(direction)
            )
        }
    }
}

fn reason_json(value: ContextInclusionReason) -> Value {
    match value {
        ContextInclusionReason::Target => json!({ "kind": "target" }),
        ContextInclusionReason::Ancestor { depth } => {
            json!({ "kind": "ancestor", "depth": depth })
        }
        ContextInclusionReason::Child { depth } => {
            json!({ "kind": "child", "depth": depth })
        }
        ContextInclusionReason::Sibling { depth } => {
            json!({ "kind": "sibling", "depth": depth })
        }
        ContextInclusionReason::Dependency { depth, relationship_type } => {
            json!({ "kind": "dependency", "depth": depth, "relationship_type": relationship_type })
        }
        ContextInclusionReason::RelatedTask { depth, relationship_type, direction } => {
            json!({
                "kind": "related_task", "depth": depth,
                "relationship_type": relationship_type, "direction": direction_name(direction),
            })
        }
    }
}

const fn format_name(value: ContextFormatArg) -> &'static str {
    match value {
        ContextFormatArg::Markdown => "markdown",
        ContextFormatArg::Json => "json",
    }
}

const fn direction_name(value: ContextRelationshipDirection) -> &'static str {
    match value {
        ContextRelationshipDirection::Incoming => "incoming",
        ContextRelationshipDirection::Outgoing => "outgoing",
    }
}

const fn relationship_name(value: minerva_domain::RelationshipType) -> &'static str {
    match value {
        minerva_domain::RelationshipType::Parent => "parent",
        minerva_domain::RelationshipType::DependsOn => "depends_on",
        minerva_domain::RelationshipType::Blocks => "blocks",
        minerva_domain::RelationshipType::RelatedTo => "related_to",
        minerva_domain::RelationshipType::Duplicates => "duplicates",
        minerva_domain::RelationshipType::Implements => "implements",
        minerva_domain::RelationshipType::References => "references",
    }
}

fn schema(err: serde_json::Error) -> minerva_domain::MinervaError {
    minerva_domain::MinervaError::SchemaError {
        path: "context-output".into(),
        reason: err.to_string(),
    }
}
