use crate::{
    MinervaLayout, TaskLock, read_task as load_task, write_task as persist_task,
};
use minerva_application::{TaskRepository, TaskWriteResult};
use minerva_domain::{ArchiveState, MinervaError, Task, TaskId, TaskVersion};
use std::{path::Path, time::SystemTime};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FilesystemTaskRepository;

impl TaskRepository for FilesystemTaskRepository {
    fn create_task(
        &self,
        root: &Path,
        task: &Task,
    ) -> Result<TaskWriteResult, MinervaError> {
        let layout = MinervaLayout::new(root);
        let _lock = TaskLock::acquire(&layout, task.id)?;
        persist_task(&layout, task)?;
        Ok(TaskWriteResult { previous_version: None, current_version: task.version })
    }

    fn read_task(&self, root: &Path, task_id: TaskId) -> Result<Task, MinervaError> {
        read_existing(&MinervaLayout::new(root), task_id)
    }

    fn update_task(
        &self,
        root: &Path,
        task: &Task,
    ) -> Result<TaskWriteResult, MinervaError> {
        let layout = MinervaLayout::new(root);
        let _lock = TaskLock::acquire(&layout, task.id)?;
        let previous = read_existing(&layout, task.id)?;
        persist_task(&layout, task)?;
        Ok(TaskWriteResult {
            previous_version: Some(previous.version),
            current_version: task.version,
        })
    }

    fn list_tasks(&self, root: &Path) -> Result<Vec<Task>, MinervaError> {
        crate::task_catalog::list_tasks(&MinervaLayout::new(root))
    }

    fn archive_task(
        &self,
        root: &Path,
        task_id: TaskId,
        version: TaskVersion,
    ) -> Result<TaskWriteResult, MinervaError> {
        let layout = MinervaLayout::new(root);
        let _lock = TaskLock::acquire(&layout, task_id)?;
        let previous = read_existing(&layout, task_id)?;
        let archived = archive(previous.clone(), &layout, version)?;
        persist_task(&layout, &archived)?;
        Ok(TaskWriteResult {
            previous_version: Some(previous.version),
            current_version: archived.version,
        })
    }

    fn resolve_task(&self, root: &Path, task_ref: &str) -> Result<Task, MinervaError> {
        crate::task_catalog::resolve_task(&MinervaLayout::new(root), task_ref)
    }

    fn search_tasks(
        &self,
        root: &Path,
        query: &str,
    ) -> Result<Vec<Task>, MinervaError> {
        crate::task_catalog::search_tasks(&MinervaLayout::new(root), query)
    }
}

fn read_existing(
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

fn archive(
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
