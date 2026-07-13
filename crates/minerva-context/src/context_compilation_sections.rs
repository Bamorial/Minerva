use crate::context_compilation_collections::{
    add_ancestors, add_dependencies, output_requirements,
};
use crate::context_compilation_render::detail_text;
use crate::context_compilation_task_sections::{add_related, add_target_sections};
use crate::{
    ContextCompilationError, ContextGraphSelection, ContextSection, ContextSectionId,
    MINERVA_EXECUTION_CONTRACT,
};
use minerva_application::{ProjectRepository, TaskRepository};
use minerva_domain::{ContextPolicy, Task};
use std::path::Path;

pub fn build_sections(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    policy: &ContextPolicy,
    target: &Task,
    selection: &ContextGraphSelection,
) -> Result<Vec<ContextSection>, ContextCompilationError> {
    let mut sections = vec![section(
        ContextSectionId::MinervaExecutionContract,
        MINERVA_EXECUTION_CONTRACT,
    )];
    add_project_instructions(&mut sections, project_repo, root, policy)?;
    push(
        &mut sections,
        ContextSectionId::TargetMetadataAndFacts,
        crate::render_target_metadata(target),
    );
    add_ancestors(&mut sections, task_repo, root, policy, selection)?;
    add_target_sections(&mut sections, task_repo, root, policy, target)?;
    add_dependencies(&mut sections, task_repo, root, policy, selection)?;
    add_related(&mut sections, policy, selection);
    push(
        &mut sections,
        ContextSectionId::OutputRequirements,
        output_requirements(target),
    );
    Ok(sections)
}

fn add_project_instructions(
    sections: &mut Vec<ContextSection>,
    repo: &impl ProjectRepository,
    root: &Path,
    policy: &ContextPolicy,
) -> Result<(), ContextCompilationError> {
    if let Some(detail) = policy.project_instructions {
        push(
            sections,
            ContextSectionId::ProjectInstructions,
            detail_text(&repo.read_project_instructions(root)?, detail),
        );
    }
    Ok(())
}
fn push(sections: &mut Vec<ContextSection>, id: ContextSectionId, body: String) {
    if let Some(section) = ContextSection::new(id, body) {
        sections.push(section);
    }
}
fn section(id: ContextSectionId, body: &str) -> ContextSection {
    ContextSection::new(id, body).unwrap()
}
