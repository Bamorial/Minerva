mod context_document;
mod context_graph_selection;
mod context_graph_selector;
mod context_inclusion_reason;
mod context_section;
mod context_section_id;
mod context_selection_item;
mod task_context_render;

pub use context_document::ContextDocument;
pub use context_graph_selection::ContextGraphSelection;
pub use context_graph_selector::ContextGraphSelector;
pub use context_inclusion_reason::{
    ContextInclusionReason, ContextRelationshipDirection,
};
pub use context_section::ContextSection;
pub use context_section_id::ContextSectionId;
pub use context_selection_item::ContextSelectionItem;
pub use task_context_render::{render_target_metadata, render_task_summary};

use minerva_application::BootstrapService;
use minerva_domain::Task;

#[must_use]
pub fn compile_workspace_context() -> String {
    let crates = BootstrapService::workspace_blueprint().crates().join(", ");
    let interfaces = BootstrapService::interface_descriptions()
        .map(|item| item.crate_name)
        .join(", ");
    format!("Workspace crates: {crates}\nInterfaces: {interfaces}")
}

#[must_use]
pub fn compile_task_context(task: &Task) -> String {
    ContextDocument::new(vec![
        ContextSection::new(
            ContextSectionId::TargetMetadataAndFacts,
            render_target_metadata(task),
        )
        .unwrap(),
    ])
    .render()
}
