use crate::{MinervaLayout, task_repository_support};
use humantime::parse_rfc3339;
use minerva_domain::{DeclarationFreshnessProbe, MinervaError, TaskEventKind, TaskId};
use std::{fs, path::Path, time::SystemTime};

pub fn read_declaration_freshness(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<DeclarationFreshnessProbe, MinervaError> {
    let task = task_repository_support::read_existing(layout, task_id)?;
    Ok(DeclarationFreshnessProbe {
        declaration_updated_at: task.declaration.updated_at,
        task_updated_at: task.updated_at,
        instructions_updated_at: instructions_updated_at(layout, task_id)?,
        relationships_updated_at: modified_at(
            &layout.relationships_file(task_id),
            false,
        )?,
        covered_commit_hash: task.declaration.commit_hash,
        current_commit_hash: None,
    })
}

fn instructions_updated_at(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<Option<SystemTime>, MinervaError> {
    let path = layout.events_file(task_id);
    if !path.exists() {
        return Ok(None);
    }
    fs::read_to_string(&path)
        .map_err(|err| schema(&path, err))?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<crate::task_event_record::TaskEventRecord>(line)
                .map_err(|err| schema(&path, err))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|event| event.kind == TaskEventKind::TaskInstructionsUpdated)
        .map(|event| {
            parse_rfc3339(&event.recorded_at).map_err(|err| schema(&path, err))
        })
        .last()
        .transpose()
}

fn modified_at(
    path: &Path,
    required: bool,
) -> Result<Option<SystemTime>, MinervaError> {
    if !path.exists() {
        return (!required)
            .then_some(None)
            .ok_or_else(|| schema(path, "file is missing"));
    }
    fs::metadata(path)
        .map_err(|err| schema(path, err))?
        .modified()
        .map(Some)
        .map_err(|err| schema(path, err))
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
