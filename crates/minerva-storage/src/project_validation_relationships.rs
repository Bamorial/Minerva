use minerva_application::ProjectValidationFinding;
use minerva_domain::{
    Relationship, Task, validate_relationships, validate_task_hierarchy,
};

pub fn validate_links(
    findings: &mut Vec<ProjectValidationFinding>,
    tasks: &[Task],
    relationships: &[Relationship],
) {
    if let Err(err) = validate_task_hierarchy(tasks) {
        findings.push(hierarchy_error(err));
    }
    if let Err(err) = validate_relationships(tasks, relationships) {
        findings.push(relationship_error(err));
    }
}

fn hierarchy_error(err: minerva_domain::MinervaError) -> ProjectValidationFinding {
    let code = match err {
        minerva_domain::MinervaError::HierarchyCycle { .. } => "hierarchy_cycle",
        _ => "missing_parent",
    };
    validation_error(code, err)
}

fn validation_error(
    code: &str,
    err: minerva_domain::MinervaError,
) -> ProjectValidationFinding {
    crate::project_validation_finding::error(
        code,
        std::path::Path::new(".minerva/tasks"),
        None,
        crate::project_validation_task_helpers::error_message(err),
    )
}

fn relationship_error(err: minerva_domain::MinervaError) -> ProjectValidationFinding {
    let code = match &err {
        minerva_domain::MinervaError::DependencyCycle { .. } => "invalid_dependency",
        minerva_domain::MinervaError::InvalidConfiguration { key, reason }
            if key == "relationships" && reason.contains("duplicate") =>
        {
            "duplicate_relationship"
        }
        _ => "invalid_dependency",
    };
    validation_error(code, err)
}
