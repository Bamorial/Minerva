use crate::context_compilation_render::{detail_text, render_related};
use crate::{
    ContextCompilationError, ContextGraphSelection, ContextInclusionReason,
    ContextSection, ContextSectionId,
};
use minerva_application::TaskRepository;
use minerva_domain::{ContextDetail, ContextPolicy, Task};
use std::path::Path;

pub fn add_target_sections(
    sections: &mut Vec<ContextSection>,
    repo: &impl TaskRepository,
    root: &Path,
    policy: &ContextPolicy,
    target: &Task,
) -> Result<(), ContextCompilationError> {
    if let Some(detail) = policy.target_task_instructions {
        push(
            sections,
            ContextSectionId::TargetInstructions,
            detail_text(&repo.read_task_instructions(root, target.id)?, detail),
        );
    }
    if let Some(detail) = policy.target_declaration {
        push(
            sections,
            ContextSectionId::TargetDeclaration,
            detail_text(&repo.read_task_declaration(root, target.id)?, detail),
        );
    }
    Ok(())
}

pub fn add_related(
    sections: &mut Vec<ContextSection>,
    policy: &ContextPolicy,
    selection: &ContextGraphSelection,
) {
    let items = selection
        .items
        .iter()
        .filter(|item| related(item.reason))
        .cloned()
        .collect::<Vec<_>>();
    let full = [
        policy.related_tasks.as_ref(),
        policy.children.as_ref(),
        policy.siblings.as_ref(),
    ]
    .into_iter()
    .flatten()
    .any(|rule| rule.detail == ContextDetail::Full);
    push(
        sections,
        ContextSectionId::RelatedTaskSummaries,
        render_related(&items, full),
    );
}

fn related(reason: ContextInclusionReason) -> bool {
    matches!(
        reason,
        ContextInclusionReason::RelatedTask { .. }
            | ContextInclusionReason::Child { .. }
            | ContextInclusionReason::Sibling { .. }
    )
}

fn push(sections: &mut Vec<ContextSection>, id: ContextSectionId, body: String) {
    if let Some(section) = ContextSection::new(id, body) {
        sections.push(section);
    }
}
