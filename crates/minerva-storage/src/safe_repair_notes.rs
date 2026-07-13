use crate::MinervaLayout;
use minerva_application::{RepairAction, RepairKind, RepairOperation, RepairSafety};
use minerva_domain::{MinervaError, TaskId};
use std::fs;

pub fn repair(
    layout: &MinervaLayout,
    dry_run: bool,
) -> Result<Vec<RepairOperation>, MinervaError> {
    let mut operations = Vec::new();
    if !layout.tasks_dir().exists() {
        return Ok(operations);
    }
    for entry in fs::read_dir(layout.tasks_dir())
        .map_err(|err| schema(layout.tasks_dir().as_path(), err))?
    {
        let entry = entry.map_err(|err| schema(layout.tasks_dir().as_path(), err))?;
        let Ok(task_id) = entry.file_name().to_string_lossy().parse::<TaskId>() else {
            continue;
        };
        let notes = layout.notes_file(task_id);
        if notes.exists() || !layout.task_file(task_id).exists() {
            continue;
        }
        if !dry_run {
            crate::write_task_notes(layout, task_id, "")?;
        }
        operations.push(RepairOperation {
            kind: RepairKind::TaskNotes,
            safety: RepairSafety::Safe,
            action: RepairAction::Create,
            path: relative(layout, &notes),
            backup_path: None,
            message: "recreated missing empty task notes file".into(),
        });
    }
    Ok(operations)
}

fn relative(layout: &MinervaLayout, path: &std::path::Path) -> String {
    path.strip_prefix(layout.root()).unwrap_or(path).display().to_string()
}

fn schema(path: &std::path::Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
