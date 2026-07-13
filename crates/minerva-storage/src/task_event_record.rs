use humantime::format_rfc3339;
use minerva_domain::{EventId, Task, TaskEventKind};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TaskEventRecord {
    pub id: EventId,
    pub kind: TaskEventKind,
    pub task_id: minerva_domain::TaskId,
    pub version: minerva_domain::TaskVersion,
    pub recorded_at: String,
}

impl TaskEventRecord {
    pub fn created(task: &Task) -> Self {
        Self {
            id: EventId::new(),
            kind: TaskEventKind::TaskCreated,
            task_id: task.id,
            version: task.version,
            recorded_at: format_rfc3339(task.updated_at).to_string(),
        }
    }
}
