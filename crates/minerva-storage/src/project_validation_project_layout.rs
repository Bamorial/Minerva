use crate::{MinervaLayout, SCHEMA_VERSION};
use minerva_application::ProjectValidationFinding;
use std::fs;

pub fn validate_layout(
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
