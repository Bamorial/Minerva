use crate::{task_event_actor::TaskEventActor, task_event_data::TaskEventData};
use humantime::format_rfc3339;
use minerva_domain::{EventId, Task, TaskEventKind, TaskId};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventRecord {
    pub id: EventId,
    pub recorded_at: String,
    pub actor: TaskEventActor,
    pub kind: TaskEventKind,
    pub task_id: TaskId,
    pub data: TaskEventData,
}

impl TaskEventRecord {
    pub fn new(
        actor: TaskEventActor,
        kind: TaskEventKind,
        task_id: TaskId,
        recorded_at: SystemTime,
        data: TaskEventData,
    ) -> Self {
        Self {
            id: EventId::new(),
            recorded_at: format_rfc3339(recorded_at).to_string(),
            actor,
            kind,
            task_id,
            data,
        }
    }

    pub fn system(task: &Task, kind: TaskEventKind, data: TaskEventData) -> Self {
        Self::new(TaskEventActor::System, kind, task.id, task.updated_at, data)
    }
}
