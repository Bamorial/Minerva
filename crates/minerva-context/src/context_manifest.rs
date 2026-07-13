use crate::{ContextSection, ContextSectionExclusion};

#[must_use]
pub fn render_context_manifest(
    sections: &[ContextSection],
    estimation_method: &str,
    total_estimated_tokens: usize,
    budget: Option<usize>,
    excluded_sections: &[ContextSectionExclusion],
) -> String {
    let mut lines = vec![
        format!("estimation_method: {estimation_method}"),
        format!("total_estimated_tokens: {total_estimated_tokens}"),
    ];
    if let Some(budget) = budget {
        lines.push(format!("budget: {budget}"));
    }
    lines.push("section_estimated_tokens:".into());
    lines.extend(sections.iter().map(|section| {
        format!("- {}: {}", section.id().heading(), section.estimated_tokens())
    }));
    if !excluded_sections.is_empty() {
        lines.push("excluded_sections:".into());
        lines.extend(excluded_sections.iter().flat_map(|section| {
            [
                format!("- section: {}", section.id().heading()),
                format!("  reason: {}", section.reason().as_str()),
                format!("  estimated_tokens: {}", section.estimated_tokens()),
            ]
        }));
    }
    lines.join("\n")
}
