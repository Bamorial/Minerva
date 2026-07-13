use crate::{
    MinervaLayout, SCHEMA_VERSION, read_project, read_project_config, read_task_types,
};
use minerva_application::ProjectValidationFinding;
use minerva_domain::{Project, StatusKey, TaskTypeKey};
use std::{collections::HashSet, fs};

pub struct ProjectValidationData {
    pub task_types: HashSet<TaskTypeKey>,
    pub statuses: HashSet<StatusKey>,
}

pub fn validate_project_files(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
) -> ProjectValidationData {
    validate_required_paths(layout, findings);
    validate_schema_marker(layout, findings);
    let task_types = load_task_types(layout, findings);
    let statuses = load_project(layout, findings);
    validate_config(layout, findings);
    ProjectValidationData { task_types, statuses }
}

fn validate_required_paths(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
) {
    for path in [
        layout.project_file(),
        layout.config_file(),
        layout.instructions_file(),
        layout.schema_version_file(),
        layout.task_types_dir(),
        layout.tasks_dir(),
    ] {
        if !path.exists() {
            findings.push(crate::project_validation_finding::error(
                "missing_file",
                &path,
                None,
                "required file is missing",
            ));
        }
    }
}

fn validate_schema_marker(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
) {
    let path = layout.schema_version_file();
    if path.exists()
        && fs::read_to_string(&path).ok().as_deref() != Some(SCHEMA_VERSION)
    {
        findings.push(crate::project_validation_finding::error(
            "schema_version",
            &path,
            None,
            format!("schema marker must be {}", SCHEMA_VERSION.trim()),
        ));
    }
}

fn load_task_types(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
) -> HashSet<TaskTypeKey> {
    read_task_types(layout).map_or_else(
        |err| {
            findings.push(project_error(
                layout.task_types_dir(),
                "malformed_yaml",
                &err,
            ));
            HashSet::new()
        },
        |items| items.into_iter().map(|item| item.name).collect(),
    )
}

fn load_project(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
) -> HashSet<StatusKey> {
    read_project(layout).map_or_else(
        |err| {
            findings.push(project_error(layout.project_file(), "malformed_yaml", &err));
            HashSet::new()
        },
        |project| statuses(&project),
    )
}

fn validate_config(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
) {
    if let Err(err) = read_project_config(layout) {
        findings.push(project_error(layout.config_file(), "malformed_yaml", &err));
    }
}

fn statuses(project: &Project) -> HashSet<StatusKey> {
    project.statuses.iter().map(|status| status.key.clone()).collect()
}

fn project_error(
    path: std::path::PathBuf,
    default_code: &str,
    err: &minerva_domain::MinervaError,
) -> ProjectValidationFinding {
    crate::project_validation_finding::error(
        crate::project_validation_task_helpers::classify(err, default_code),
        &path,
        None,
        crate::project_validation_task_helpers::error_message(err.clone()),
    )
}
