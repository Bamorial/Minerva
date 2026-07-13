use minerva_application::{ProjectValidationFinding, ValidationSeverity};
use std::path::Path;

pub fn error(
    code: &str,
    path: &Path,
    task_ref: Option<&str>,
    message: impl Into<String>,
) -> ProjectValidationFinding {
    finding(ValidationSeverity::Error, code, path, task_ref, message)
}

pub fn warning(
    code: &str,
    path: &Path,
    task_ref: Option<&str>,
    message: impl Into<String>,
) -> ProjectValidationFinding {
    finding(ValidationSeverity::Warning, code, path, task_ref, message)
}

pub fn info(
    code: &str,
    path: &Path,
    task_ref: Option<&str>,
    message: impl Into<String>,
) -> ProjectValidationFinding {
    finding(ValidationSeverity::Information, code, path, task_ref, message)
}

fn finding(
    severity: ValidationSeverity,
    code: &str,
    path: &Path,
    task_ref: Option<&str>,
    message: impl Into<String>,
) -> ProjectValidationFinding {
    ProjectValidationFinding {
        severity,
        code: code.into(),
        path: path.display().to_string(),
        task_ref: task_ref.map(str::to_string),
        message: message.into(),
    }
}
