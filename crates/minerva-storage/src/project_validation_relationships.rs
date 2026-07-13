use minerva_application::ProjectValidationFinding;
use minerva_domain::{
    Relationship, Task, validate_relationships, validate_task_hierarchy,
};
use std::collections::HashSet;

pub fn validate_links(
    findings: &mut Vec<ProjectValidationFinding>,
    tasks: &[Task],
    relationships: &[Relationship],
) {
    if let Err(err) = validate_task_hierarchy(tasks) {
        findings.extend(hierarchy_errors(tasks, &err));
    }
    if let Err(err) = validate_relationships(tasks, relationships) {
        findings.extend(relationship_errors(tasks, relationships, &err));
    }
}

fn hierarchy_errors(
    tasks: &[Task],
    err: &minerva_domain::MinervaError,
) -> Vec<ProjectValidationFinding> {
    match err {
        minerva_domain::MinervaError::HierarchyCycle { parent, child } => {
            vec![
                task_error("hierarchy_cycle", parent, err.to_string()),
                task_error("hierarchy_cycle", child, err.to_string()),
            ]
        }
        minerva_domain::MinervaError::TaskNotFound { task_ref } => tasks
            .iter()
            .find(|task| task.parent_id.is_some_and(|id| id.to_string() == *task_ref))
            .map(|task| {
                task_error("missing_parent", &task.id.to_string(), err.to_string())
            })
            .into_iter()
            .collect(),
        _ => vec![validation_error("missing_parent", err)],
    }
}

fn validation_error(
    code: &str,
    err: &minerva_domain::MinervaError,
) -> ProjectValidationFinding {
    crate::project_validation_finding::error(
        code,
        std::path::Path::new(".minerva/tasks"),
        None,
        crate::project_validation_task_helpers::error_message(err),
    )
}

fn relationship_errors(
    tasks: &[Task],
    relationships: &[Relationship],
    err: &minerva_domain::MinervaError,
) -> Vec<ProjectValidationFinding> {
    match err {
        minerva_domain::MinervaError::DependencyCycle { task, depends_on } => vec![
            task_error("invalid_dependency", task, err.to_string()),
            task_error("invalid_dependency", depends_on, err.to_string()),
        ],
        minerva_domain::MinervaError::InvalidConfiguration { key, reason }
            if key == "relationships" && reason.contains("duplicate") =>
        {
            duplicate_errors(relationships, err.to_string())
        }
        _ => missing_relationship_scope(tasks, relationships)
            .map(|task_ref| {
                task_error("invalid_dependency", &task_ref, err.to_string())
            })
            .into_iter()
            .collect(),
    }
}

fn duplicate_errors(
    relationships: &[Relationship],
    message: String,
) -> Vec<ProjectValidationFinding> {
    let mut seen = HashSet::new();
    relationships
        .iter()
        .find(|item| !seen.insert(item.semantic_key()))
        .map(|item| {
            vec![
                task_error(
                    "duplicate_relationship",
                    &item.source_task.to_string(),
                    message.clone(),
                ),
                task_error(
                    "duplicate_relationship",
                    &item.target_task.to_string(),
                    message,
                ),
            ]
        })
        .unwrap_or_default()
}

fn missing_relationship_scope(
    tasks: &[Task],
    relationships: &[Relationship],
) -> Option<String> {
    let known = tasks.iter().map(|task| task.id).collect::<HashSet<_>>();
    relationships.iter().find_map(|item| {
        (!known.contains(&item.source_task) || !known.contains(&item.target_task))
            .then(|| item.source_task.to_string())
    })
}

fn task_error(code: &str, task_ref: &str, message: String) -> ProjectValidationFinding {
    crate::project_validation_finding::error(
        code,
        std::path::Path::new(".minerva/tasks"),
        Some(task_ref),
        message,
    )
}
