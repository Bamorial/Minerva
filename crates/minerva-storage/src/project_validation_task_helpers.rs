use crate::MinervaLayout;
use minerva_application::ProjectValidationFinding;
use minerva_domain::{MinervaError, TaskId};

pub fn task_error(
    layout: &MinervaLayout,
    task_id: TaskId,
    default_code: &str,
    task_ref: &str,
    err: &MinervaError,
) -> ProjectValidationFinding {
    let path = layout.task_file(task_id);
    crate::project_validation_finding::error(
        classify(err, default_code),
        &path,
        Some(task_ref),
        error_message(err),
    )
}

pub fn classify<'a>(err: &MinervaError, default_code: &'a str) -> &'a str {
    match err {
        MinervaError::SchemaError { reason, .. }
        | MinervaError::InvalidConfiguration { key: _, reason }
            if reason == "required file is missing" =>
        {
            "missing_file"
        }
        MinervaError::SchemaError { reason, .. }
            if reason.contains("schema_version")
                || reason.contains("unsupported schema version") =>
        {
            "schema_version"
        }
        MinervaError::InvalidConfiguration { key, .. } if key == "schema_version" => {
            "schema_version"
        }
        _ => default_code,
    }
}

pub fn error_message(err: &MinervaError) -> String {
    match err {
        MinervaError::SchemaError { reason, .. }
        | MinervaError::InvalidConfiguration { reason, .. } => reason.clone(),
        MinervaError::TaskNotFound { task_ref } => {
            format!("task `{task_ref}` was not found")
        }
        other => other.to_string(),
    }
}
