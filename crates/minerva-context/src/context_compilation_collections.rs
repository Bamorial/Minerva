use crate::context_compilation_render::{
    declaration_text, detail_text, render_collection,
};
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
    let mut requirements = if target.facts.acceptance_checks.is_empty() {
        Vec::new()
    } else {
        target
            .facts
            .acceptance_checks
            .iter()
            .map(|check| format!("- {check}"))
            .collect::<Vec<_>>()
    };
    requirements
        .push("The agent must complete the declaration before finishing.".into());
    requirements.push(format!(
        "Declaration path: `.minerva/tasks/{}/declaration.md`",
        target.id
    ));
    requirements.join("\n")
}

fn read_collection(
    repo: &impl TaskRepository,
    root: &Path,
    selection: &ContextGraphSelection,
    keep: fn(ContextInclusionReason) -> bool,
    instructions: bool,
    detail: ContextDetail,
) -> Result<String, ContextCompilationError> {
    let mut items = Vec::new();
    for item in selection.items.iter().filter(|item| keep(item.reason)) {
        let text = if instructions {
            repo.read_task_instructions(root, item.task.id)?
        } else {
            repo.read_task_declaration(root, item.task.id)?
        };
        let body = if instructions {
            Some(detail_text(&text, detail))
        } else {
            declaration_text(&text, detail)
        };
        let Some(body) = body else {
            continue;
        };
        items.push((item, body));
    }
    Ok(render_collection(&items))
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
