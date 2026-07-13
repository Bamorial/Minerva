use crate::context_compilation_render::{detail_text, render_collection};
use crate::{
    ContextCompilationError, ContextGraphSelection, ContextInclusionReason,
    ContextSection, ContextSectionId,
};
use minerva_application::TaskRepository;
use minerva_domain::{ContextDetail, ContextPolicy, Task};
use std::path::Path;

pub fn add_ancestors(
    sections: &mut Vec<ContextSection>,
    repo: &impl TaskRepository,
    root: &Path,
    policy: &ContextPolicy,
    selection: &ContextGraphSelection,
) -> Result<(), ContextCompilationError> {
    let Some(rule) = &policy.ancestors else {
        return Ok(());
    };
    if rule.detail == ContextDetail::Full {
        push(
            sections,
            ContextSectionId::AncestorInstructions,
            read_collection(repo, root, selection, ancestor, true, rule.detail)?,
        );
    }
    push(
        sections,
        ContextSectionId::AncestorDeclarations,
        read_collection(repo, root, selection, ancestor, false, rule.detail)?,
    );
    Ok(())
}

pub fn add_dependencies(
    sections: &mut Vec<ContextSection>,
    repo: &impl TaskRepository,
    root: &Path,
    policy: &ContextPolicy,
    selection: &ContextGraphSelection,
) -> Result<(), ContextCompilationError> {
    let Some(rule) = &policy.dependencies else {
        return Ok(());
    };
    push(
        sections,
        ContextSectionId::DependencyDeclarations,
        read_collection(repo, root, selection, dependency, false, rule.detail)?,
    );
    Ok(())
}

pub fn output_requirements(target: &Task) -> String {
    if target.facts.acceptance_checks.is_empty() {
        return "Follow the target task instructions and satisfy the declaration before completion.".into();
    }
    target
        .facts
        .acceptance_checks
        .iter()
        .map(|check| format!("- {check}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn read_collection(
    repo: &impl TaskRepository,
    root: &Path,
    selection: &ContextGraphSelection,
    keep: fn(ContextInclusionReason) -> bool,
    instructions: bool,
    detail: ContextDetail,
) -> Result<String, ContextCompilationError> {
    let items = selection.items.iter().filter(|item| keep(item.reason)).map(|item| {
        let text = if instructions {
            repo.read_task_instructions(root, item.task.id)
        } else {
            repo.read_task_declaration(root, item.task.id)
        }?;
        Ok((item, detail_text(&text, detail)))
    });
    Ok(render_collection(&items.collect::<Result<Vec<_>, ContextCompilationError>>()?))
}

fn ancestor(reason: ContextInclusionReason) -> bool {
    matches!(reason, ContextInclusionReason::Ancestor { .. })
}
fn dependency(reason: ContextInclusionReason) -> bool {
    matches!(reason, ContextInclusionReason::Dependency { .. })
}
fn push(sections: &mut Vec<ContextSection>, id: ContextSectionId, body: String) {
    if let Some(section) = ContextSection::new(id, body) {
        sections.push(section);
    }
}
