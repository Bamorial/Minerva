use crate::response::CommandOutput;
use minerva_application::{RepairAction, RepairResult, ValidationSeverity};
use serde_json::json;

pub fn render(result: &RepairResult, dry_run: bool) -> CommandOutput {
    let validation_json = result.validation.as_ref().map(|validation| {
        json!({
            "summary": validation.summary(),
            "findings": validation.findings,
        })
    });
    let header = if result.operations.is_empty() {
        "repair: no safe repairs needed".to_string()
    } else {
        format!("repair: {} safe change(s)", result.operations.len())
    };
    let mut lines = vec![header];
    lines.extend(result.operations.iter().map(|item| line(item, dry_run)));
    lines.extend(
        result
            .issues
            .iter()
            .map(|item| format!("issue {} {}: {}", item.code, item.path, item.message)),
    );
    if let Some(validation) = &result.validation {
        let summary = validation.summary();
        lines.push(format!(
            "validate project: {} error(s), {} warning(s), {} info finding(s)",
            summary.errors, summary.warnings, summary.information
        ));
        lines.extend(validation.findings.iter().map(validation_line));
    }
    CommandOutput::with_json(
        lines.join("\n"),
        json!({
            "dry_run": dry_run,
            "operations": result.operations,
            "issues": result.issues,
            "validation": validation_json,
        }),
    )
}

fn line(item: &minerva_application::RepairOperation, dry_run: bool) -> String {
    let action = match (dry_run, item.action) {
        (true, RepairAction::Create) => "would create",
        (true, RepairAction::Update) => "would update",
        (true, RepairAction::Remove) => "would remove",
        (false, RepairAction::Create) => "created",
        (false, RepairAction::Update) => "updated",
        (false, RepairAction::Remove) => "removed",
    };
    let backup = item
        .backup_path
        .as_deref()
        .map_or_else(String::new, |path| format!(" backup={path}"));
    format!("safe {:?} {action} {}:{}{}", item.kind, item.path, item.message, backup)
}

fn validation_line(item: &minerva_application::ProjectValidationFinding) -> String {
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
