use humantime::format_rfc3339;
use minerva_domain::{EventId, Task, TaskEventKind};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TaskEventRecord {
    pub id: EventId,
    pub kind: TaskEventKind,
    pub task_id: minerva_domain::TaskId,
    pub version: minerva_domain::TaskVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_parent_id: Option<minerva_domain::TaskId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_parent_id: Option<minerva_domain::TaskId>,
    pub recorded_at: String,
}

impl TaskEventRecord {
    pub fn created(task: &Task) -> Self {
        Self {
            id: EventId::new(),
            kind: TaskEventKind::TaskCreated,
            task_id: task.id,
            version: task.version,
            from_parent_id: None,
            to_parent_id: None,
            recorded_at: format_rfc3339(task.updated_at).to_string(),
        }
    }

    pub fn moved(task: &Task, from_parent_id: Option<minerva_domain::TaskId>) -> Self {
        Self {
            id: EventId::new(),
            kind: TaskEventKind::TaskMoved,
            task_id: task.id,
            version: task.version,
            from_parent_id,
            to_parent_id: task.parent_id,
            recorded_at: format_rfc3339(task.updated_at).to_string(),
        }
    }

    pub fn instructions_updated(task: &Task) -> Self {
        Self {
            id: EventId::new(),
            kind: TaskEventKind::TaskInstructionsUpdated,
            task_id: task.id,
            version: task.version,
            from_parent_id: None,
            to_parent_id: None,
            recorded_at: format_rfc3339(task.updated_at).to_string(),
        }
    }

    pub fn declaration_updated(task: &Task) -> Self {
        Self {
            id: EventId::new(),
            kind: TaskEventKind::TaskDeclarationUpdated,
            task_id: task.id,
            version: task.version,
            from_parent_id: None,
            to_parent_id: None,
            recorded_at: format_rfc3339(task.updated_at).to_string(),
        }
    }
}
