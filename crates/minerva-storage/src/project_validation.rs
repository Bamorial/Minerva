use crate::MinervaLayout;
use minerva_application::ProjectValidationResult;
use minerva_domain::MinervaError;
use std::path::Path;

pub fn validate_project_state(
    root: &Path,
) -> Result<ProjectValidationResult, MinervaError> {
    let layout = MinervaLayout::new(root);
    let mut findings = Vec::new();
    let project = crate::project_validation_project::validate_project_files(
        &layout,
        &mut findings,
    );
    let tasks = crate::project_validation_tasks::validate_tasks(
        &layout,
        &mut findings,
        &project.task_types,
        &project.statuses,
    );
    crate::project_validation_relationships::validate_links(
        &mut findings,
        &tasks.tasks,
        &tasks.relationships,
    );
    crate::project_validation_index::validate_index(&layout, &mut findings);
    findings.sort_by_key(sort_key);
    Ok(ProjectValidationResult { findings })
}

fn sort_key(
    finding: &minerva_application::ProjectValidationFinding,
) -> (u8, String, String, String) {
    let task_ref = finding.task_ref.clone().unwrap_or_default();
    (severity_rank(finding), finding.code.clone(), task_ref, finding.path.clone())
}

fn severity_rank(finding: &minerva_application::ProjectValidationFinding) -> u8 {
    match finding.severity {
        minerva_application::ValidationSeverity::Error => 0,
        minerva_application::ValidationSeverity::Warning => 1,
        minerva_application::ValidationSeverity::Information => 2,
    }
}
