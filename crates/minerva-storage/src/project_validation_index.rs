use crate::{MinervaLayout, TaskIndexStatus, task_index_status};
use minerva_application::ProjectValidationFinding;

pub fn validate_index(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
) {
    let path = layout.task_index_file();
    match task_index_status(layout) {
        Ok(TaskIndexStatus::Fresh) => {
            findings.push(crate::project_validation_finding::info(
                "stale_index",
                &path,
                None,
                "task index is current",
            ));
        }
        Ok(TaskIndexStatus::Missing) => {
            findings.push(crate::project_validation_finding::warning(
                "stale_index",
                &path,
                None,
                "task index is missing",
            ));
        }
        Ok(TaskIndexStatus::Stale) => {
            findings.push(crate::project_validation_finding::warning(
                "stale_index",
                &path,
                None,
                "task index is stale",
            ));
        }
        Err(err) => findings.push(crate::project_validation_finding::warning(
            "stale_index",
            &path,
            None,
            crate::project_validation_task_helpers::error_message(&err),
        )),
    }
}
