use crate::{MinervaLayout, atomic_replace, task_event_record::TaskEventRecord};
use minerva_domain::{EventId, MinervaError, StatusKey, Task, TaskId};
use std::fs;

pub fn append_created_event(
    layout: &MinervaLayout,
    task: &Task,
) -> Result<EventId, MinervaError> {
    append_event(layout, task, TaskEventRecord::created(task))
}

pub fn append_moved_event(
    layout: &MinervaLayout,
    task: &Task,
    from_parent_id: Option<TaskId>,
) -> Result<EventId, MinervaError> {
    append_event(layout, task, TaskEventRecord::moved(task, from_parent_id))
}

pub fn append_instructions_updated_event(
    layout: &MinervaLayout,
    task: &Task,
) -> Result<EventId, MinervaError> {
    append_event(layout, task, TaskEventRecord::instructions_updated(task))
}

pub fn append_declaration_updated_event(
    layout: &MinervaLayout,
    task: &Task,
) -> Result<EventId, MinervaError> {
    append_event(layout, task, TaskEventRecord::declaration_updated(task))
}

pub fn append_status_updated_event(
    layout: &MinervaLayout,
    task: &Task,
    from_status: StatusKey,
    completion_override: bool,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        task,
        TaskEventRecord::status_updated(task, from_status, completion_override),
    )
}

fn append_event(
    layout: &MinervaLayout,
    task: &Task,
    event: TaskEventRecord,
) -> Result<EventId, MinervaError> {
    let path = layout.events_file(task.id);
    let mut contents = fs::read_to_string(&path).unwrap_or_default();
    let line = serde_json::to_string(&event).map_err(|err| schema(&path, err))?;
    contents.push_str(&line);
    contents.push('\n');
    atomic_replace(&path, contents.as_bytes()).map_err(|err| schema(&path, err))?;
    Ok(event.id)
}

fn schema(path: &std::path::Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
