mod context_budget;
mod context_budget_error;
mod context_budget_report;
mod context_compilation_collections;
mod context_compilation_error;
mod context_compilation_freshness;
mod context_compilation_render;
mod context_compilation_request;
mod context_compilation_result;
mod context_compilation_sections;
mod context_compilation_service;
mod context_compilation_task_sections;
mod context_document;
mod context_exclusion_reason;
mod context_execution_contract;
mod context_graph_selection;
mod context_graph_selector;
mod context_inclusion_reason;
mod context_manifest;
mod context_section;
mod context_section_exclusion;
mod context_section_id;
mod context_selection_item;
mod task_context_render;
mod token_estimator;

pub use context_budget_error::ContextBudgetError;
pub use context_budget_report::ContextBudgetReport;
pub use context_compilation_error::ContextCompilationError;
pub use context_compilation_request::ContextCompilationRequest;
pub use context_compilation_result::ContextCompilationResult;
pub use context_compilation_service::ContextCompilationService;
pub use context_document::ContextDocument;
pub use context_exclusion_reason::ContextExclusionReason;
pub use context_execution_contract::MINERVA_EXECUTION_CONTRACT;
pub use context_graph_selection::ContextGraphSelection;
pub use context_graph_selector::ContextGraphSelector;
pub use context_inclusion_reason::{
    ContextInclusionReason, ContextRelationshipDirection,
};
pub use context_manifest::{ContextInputHash, ContextManifest, ContextManifestEntry};
pub use context_section::ContextSection;
pub use context_section_exclusion::ContextSectionExclusion;
pub use context_section_id::ContextSectionId;
pub use context_selection_item::ContextSelectionItem;
pub use task_context_render::{render_target_metadata, render_task_summary};
pub use token_estimator::{
    ApproximateTokenEstimator, MIXED_TOKEN_ESTIMATION_METHOD, TokenEstimator,
};

use minerva_application::BootstrapService;
use minerva_domain::Task;

#[must_use]
pub fn compile_workspace_context() -> String {
    let crates = BootstrapService::workspace_blueprint().crates().join(", ");
    let interfaces = BootstrapService::interface_descriptions()
        .map(|item| item.crate_name)
        .join(", ");
    ContextDocument::new(vec![
        ContextSection::new(
            ContextSectionId::ProjectInstructions,
            format!("Workspace crates: {crates}\nInterfaces: {interfaces}"),
        )
        .unwrap(),
    ])
    .render_with_manifest()
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
    .render_with_manifest()
}

pub fn compile_task_context_with_budget(
    task: &Task,
    budget: usize,
) -> Result<String, ContextBudgetError> {
    ContextDocument::new(vec![
        ContextSection::new(
            ContextSectionId::TargetMetadataAndFacts,
            render_target_metadata(task),
        )
        .unwrap(),
    ])
    .enforce_budget(budget)
    .map(|report| report.render_with_manifest())
}
