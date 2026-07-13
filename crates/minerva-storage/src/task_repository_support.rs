use crate::{MinervaLayout, read_task as load_task};
use minerva_domain::{ArchiveState, MinervaError, Task, TaskId, TaskVersion};
use std::time::SystemTime;

pub fn read_existing(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<Task, MinervaError> {
    layout
        .task_file(task_id)
        .exists()
        .then(|| load_task(layout, task_id))
        .unwrap_or_else(|| {
            Err(MinervaError::TaskNotFound { task_ref: task_id.to_string() })
        })
}

pub fn archive(
    task: Task,
    layout: &MinervaLayout,
    version: TaskVersion,
) -> Result<Task, MinervaError> {
    if task.archive_state == ArchiveState::Archived {
        return Err(MinervaError::InvalidConfiguration {
            key: "archive_state".into(),
            reason: "task is already archived".into(),
        });
    }
    if task.version != version {
        return Err(MinervaError::VersionConflict {
            path: layout.task_file(task.id).display().to_string(),
            expected: task.version.get().to_string(),
            actual: version.get().to_string(),
        });
    }
    Ok(Task {
        archive_state: ArchiveState::Archived,
        updated_at: SystemTime::now(),
        version: task.version.next(),
        ..task
    })
}
