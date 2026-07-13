use crate::ContextSection;

#[must_use]
pub fn render_context_manifest(
    sections: &[ContextSection],
    estimation_method: &str,
    total_estimated_tokens: usize,
) -> String {
    let mut lines = vec![
        format!("estimation_method: {estimation_method}"),
        format!("total_estimated_tokens: {total_estimated_tokens}"),
        "section_estimated_tokens:".into(),
    ];
    lines.extend(sections.iter().map(|section| {
        format!("- {}: {}", section.id().heading(), section.estimated_tokens())
    }));
    lines.join("\n")
}
