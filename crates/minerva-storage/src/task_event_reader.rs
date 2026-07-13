use crate::{MinervaLayout, task_event_record::TaskEventRecord};
use minerva_domain::{MinervaError, TaskId};
use std::{fs, path::Path};

pub fn read_task_events(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<Vec<TaskEventRecord>, MinervaError> {
    let path = layout.events_file(task_id);
    if !path.exists() {
        return Ok(Vec::new());
    }
    fs::read_to_string(&path)
        .map_err(|err| schema(&path, err))?
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            serde_json::from_str(line)
                .map_err(|err| schema(&path, format!("line {}: {err}", index + 1)))
        })
        .collect()
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
