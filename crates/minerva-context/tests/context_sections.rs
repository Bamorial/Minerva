mod support;

use minerva_context::{
    ContextDocument, ContextSection, ContextSectionId, compile_task_context,
    compile_workspace_context,
};
use support::task;

#[test]
fn workspace_context_compilation_is_deterministic() {
    let context = compile_workspace_context();
    assert!(context.contains("minerva-domain"));
    assert!(context.contains("minerva-mcp"));
}

#[test]
fn task_context_includes_structured_facts_under_stable_heading() {
    let context = compile_task_context(&task());
    assert!(context.starts_with("## Target Metadata and Facts"));
    assert!(context.contains("facts.modules: minerva-domain"));
    assert!(context.contains("facts.migrations_required: true"));
}

#[test]
fn context_document_renders_sections_in_recommended_order() {
    let document = ContextDocument::new(
        [
            section(ContextSectionId::ContextManifestSummary, "11"),
            section(ContextSectionId::TargetDeclaration, "7"),
            section(ContextSectionId::ProjectInstructions, "2"),
            section(ContextSectionId::DependencyDeclarations, "8"),
            section(ContextSectionId::MinervaExecutionContract, "1"),
            section(ContextSectionId::RelatedTaskSummaries, "9"),
            section(ContextSectionId::OutputRequirements, "10"),
            section(ContextSectionId::TargetInstructions, "6"),
            section(ContextSectionId::TargetMetadataAndFacts, "3"),
            section(ContextSectionId::AncestorDeclarations, "5"),
            section(ContextSectionId::AncestorInstructions, "4"),
        ]
        .into_iter()
        .flatten()
        .collect(),
    );
    assert_eq!(document.render(), expected_order());
}

#[test]
fn context_document_skips_missing_optional_sections() {
    let document = ContextDocument::new(
        [
            section(ContextSectionId::ProjectInstructions, "Use Rust."),
            section(ContextSectionId::OutputRequirements, ""),
            section(ContextSectionId::TargetDeclaration, "Implementation note."),
        ]
        .into_iter()
        .flatten()
        .collect(),
    );
    assert_eq!(
        document.render(),
        "## Project Instructions\n\nUse Rust.\n\n## Target Declaration\n\nImplementation note."
    );
}

fn section(id: ContextSectionId, body: &str) -> Option<ContextSection> {
    ContextSection::new(id, body)
}

fn expected_order() -> &'static str {
    "## Minerva Execution Contract\n\n1\n\n## Project Instructions\n\n2\n\n## Target Metadata and Facts\n\n3\n\n## Ancestor Instructions\n\n4\n\n## Ancestor Declarations\n\n5\n\n## Target Instructions\n\n6\n\n## Target Declaration\n\n7\n\n## Dependency Declarations\n\n8\n\n## Related Task Summaries\n\n9\n\n## Output Requirements\n\n10\n\n## Context Manifest Summary\n\n11"
}
