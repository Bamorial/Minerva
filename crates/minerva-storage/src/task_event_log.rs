use crate::{MinervaLayout, task_event_record::TaskEventRecord};
use minerva_domain::{MinervaError, TaskId};
use std::{fs, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEventLog {
    pub events: Vec<TaskEventRecord>,
    pub issues: Vec<TaskEventLogIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEventLogIssue {
    pub line: usize,
    pub reason: String,
}

pub fn read_task_event_log(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<TaskEventLog, MinervaError> {
    let path = layout.events_file(task_id);
    if !path.exists() {
        return Ok(TaskEventLog { events: Vec::new(), issues: Vec::new() });
    }
    let contents = fs::read_to_string(&path).map_err(|err| schema(&path, err))?;
    let mut events = Vec::new();
    let mut issues = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str(line) {
            Ok(event) => events.push(event),
            Err(err) => issues
                .push(TaskEventLogIssue { line: index + 1, reason: err.to_string() }),
        }
    }
    Ok(TaskEventLog { events, issues })
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
