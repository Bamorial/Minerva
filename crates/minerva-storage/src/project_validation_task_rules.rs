use minerva_application::ProjectValidationFinding;
use minerva_domain::{StatusKey, Task, TaskTypeKey};
use std::collections::HashSet;

pub fn validate_task_model(
    findings: &mut Vec<ProjectValidationFinding>,
    task_types: &HashSet<TaskTypeKey>,
    statuses: &HashSet<StatusKey>,
    task: &Task,
) {
    let path = std::path::PathBuf::from(".minerva/tasks")
        .join(task.id.to_string())
        .join("task.yaml");
    if !task_types.contains(&task.task_type) {
        findings.push(crate::project_validation_finding::error(
            "invalid_task_type",
            &path,
            Some(&task.id.to_string()),
            format!("unknown task type `{}`", task.task_type),
        ));
    }
    if !statuses.contains(&task.status) {
        findings.push(crate::project_validation_finding::error(
            "illegal_status",
            &path,
            Some(&task.id.to_string()),
            format!("unknown status `{}`", task.status),
        ));
    }
}
