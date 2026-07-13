use crate::{MinervaLayout, read_task_declaration, read_task_event_log};
use minerva_application::ProjectValidationFinding;
use minerva_domain::TaskId;

pub fn validate_task_files(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
    task_id: TaskId,
    task_ref: &str,
) {
    for path in required_paths(layout, task_id) {
        if !path.exists() {
            findings.push(crate::project_validation_finding::error(
                "missing_file",
                &path,
                Some(task_ref),
                "required file is missing",
            ));
        }
    }
    validate_declaration(layout, findings, task_id, task_ref);
    validate_events(layout, findings, task_id, task_ref);
}

fn required_paths(layout: &MinervaLayout, task_id: TaskId) -> [std::path::PathBuf; 5] {
    [
        layout.task_file(task_id),
        layout.task_instructions_file(task_id),
        layout.declaration_file(task_id),
        layout.notes_file(task_id),
        layout.events_file(task_id),
    ]
}

fn validate_declaration(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
    task_id: TaskId,
    task_ref: &str,
) {
    let path = layout.declaration_file(task_id);
    if path.exists()
        && let Err(err) = read_task_declaration(layout, task_id)
    {
        findings.push(crate::project_validation_task_helpers::task_error(
            layout,
            task_id,
            "invalid_declaration",
            task_ref,
            &err,
        ));
    }
}

fn validate_events(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
    task_id: TaskId,
    task_ref: &str,
) {
    let path = layout.events_file(task_id);
    if !path.exists() {
        return;
    }
    match read_task_event_log(layout, task_id) {
        Ok(log) => findings.extend(log.issues.into_iter().map(|issue| {
            crate::project_validation_finding::error(
                "malformed_event_log",
                &path,
                Some(task_ref),
                format!("line {}: {}", issue.line, issue.reason),
            )
        })),
        Err(err) => findings.push(crate::project_validation_task_helpers::task_error(
            layout,
            task_id,
            "malformed_event_log",
            task_ref,
            &err,
        )),
    }
}
