use crate::response::CommandOutput;
use minerva_application::{
    ProjectMigrationAction, ProjectMigrationResult, ProjectValidationResult,
    RebuildAction, RebuildResult,
};
use serde_json::json;

pub fn render(
    migration: &ProjectMigrationResult,
    rebuild: Option<&RebuildResult>,
    validation: Option<&ProjectValidationResult>,
    dry_run: bool,
) -> CommandOutput {
    let mut lines = vec![header(migration, dry_run)];
    lines.extend(migration.steps.iter().map(step_line));
    lines.extend(migration.steps.iter().flat_map(|step| {
        step.operations.iter().map(|item| operation_line(item, dry_run))
    }));
    if let Some(rebuild) = rebuild {
        lines.push(render_rebuild(rebuild));
    }
    if let Some(validation) = validation {
        lines.push(crate::validate_output::render(validation, None).text);
    }
    CommandOutput::with_json(
        lines.join("\n"),
        json!({
            "dry_run": dry_run,
            "migration": migration,
            "rebuild": rebuild.map(|item| json!({
                "index_path": item.index_path,
                "index_action": rebuild_action(item.index_action),
                "task_errors": item.task_errors.iter().map(|error| json!({
                    "task_ref": error.task_ref,
                    "path": error.path,
                    "reason": error.reason,
                })).collect::<Vec<_>>(),
            })),
            "validation": validation.map(|item| json!({
                "summary": item.summary(),
                "findings": item.findings,
            })),
        }),
    )
}

fn header(result: &ProjectMigrationResult, dry_run: bool) -> String {
    if result.is_current() {
        "migrate: project is current".into()
    } else if dry_run {
        format!("migrate: would apply {} step(s)", result.steps.len())
    } else {
        format!("migrate: applied {} step(s)", result.steps.len())
    }
}

fn step_line(step: &minerva_application::ProjectMigrationStep) -> String {
    format!("step {} {}->{}", step.name, step.from_version, step.to_version)
}

fn operation_line(
    item: &minerva_application::ProjectMigrationOperation,
    dry_run: bool,
) -> String {
    let action = match (dry_run, item.action) {
        (true, ProjectMigrationAction::Create) => "would create",
        (true, ProjectMigrationAction::Update) => "would update",
        (false, ProjectMigrationAction::Create) => "created",
        (false, ProjectMigrationAction::Update) => "updated",
    };
    let backup = item
        .backup_path
        .as_deref()
        .map_or_else(String::new, |path| format!(" backup={path}"));
    format!("{action} {}: {}{}", item.path, item.message, backup)
}

fn render_rebuild(result: &RebuildResult) -> String {
    let action = match result.index_action {
        RebuildAction::Create | RebuildAction::Update => "wrote",
        RebuildAction::NoChange => "kept",
    };
    let mut lines = vec![format!("rebuild: {action} {}", result.index_path)];
    lines.extend(result.task_errors.iter().map(|error| {
        format!("invalid task {} at {}: {}", error.task_ref, error.path, error.reason)
    }));
    lines.join("\n")
}

fn rebuild_action(action: RebuildAction) -> &'static str {
    match action {
        RebuildAction::Create => "create",
        RebuildAction::Update => "update",
        RebuildAction::NoChange => "no_change",
    }
}
