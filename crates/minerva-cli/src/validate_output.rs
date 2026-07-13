use crate::response::CommandOutput;
use minerva_application::{ProjectValidationResult, ValidationSeverity};
use serde_json::json;

pub fn render(
    result: &ProjectValidationResult,
    task_ref: Option<&str>,
) -> CommandOutput {
    let summary = result.summary();
    let scope: String = task_ref.map_or_else(|| "project".to_string(), str::to_string);
    let header = format!(
        "validate {scope}: {} error(s), {} warning(s), {} info finding(s)",
        summary.errors, summary.warnings, summary.information
    );
    let text = if result.findings.is_empty() {
        format!("{header}\nno findings")
    } else {
        format!("{header}\n{}", lines(&result.findings).join("\n"))
    };
    CommandOutput::with_json(
        text,
        json!({
            "scope": { "task_ref": task_ref },
            "summary": summary,
            "findings": result.findings,
        }),
    )
}

fn lines(findings: &[minerva_application::ProjectValidationFinding]) -> Vec<String> {
    findings.iter().map(line).collect()
}

fn line(item: &minerva_application::ProjectValidationFinding) -> String {
    let severity = match item.severity {
        ValidationSeverity::Error => "error",
        ValidationSeverity::Warning => "warning",
        ValidationSeverity::Information => "info",
    };
    let task = item
        .task_ref
        .as_deref()
        .map_or_else(String::new, |value| format!(" [{value}]"));
    format!("{severity} {} {}{}: {}", item.code, item.path, task, item.message)
}
