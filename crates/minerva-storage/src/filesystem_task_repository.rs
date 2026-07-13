use crate::{
    MinervaLayout, TaskLock, append_created_event, task_repository_support,
    write_task as persist_task, write_task_declaration, write_task_instructions,
    write_task_notes,
};
use minerva_application::{TaskCreateRecord, TaskRepository, TaskWriteResult};
use minerva_domain::{MinervaError, Task, TaskId, TaskIdAllocator, TaskVersion};
use std::path::Path;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FilesystemTaskRepository;

impl TaskRepository for FilesystemTaskRepository {
    fn next_task_id(&self, root: &Path) -> Result<TaskId, MinervaError> {
        let last = self
            .list_tasks(root)?
            .into_iter()
            .map(|task| task.id.sequence().get())
            .max()
            .unwrap_or(0);
        Ok(TaskIdAllocator::new(last).next_id())
    }

    fn create_task(
        &self,
        root: &Path,
        record: &TaskCreateRecord,
    ) -> Result<TaskWriteResult, MinervaError> {
        let layout = MinervaLayout::new(root);
        let _lock = TaskLock::acquire(&layout, record.task.id)?;
        persist_task(&layout, &record.task)?;
        write_task_instructions(&layout, record.task.id, &record.instructions)?;
        write_task_declaration(&layout, record.task.id, &record.declaration)?;
        write_task_notes(&layout, record.task.id, "")?;
        let event_id = append_created_event(&layout, &record.task)?;
        Ok(TaskWriteResult {
            previous_version: None,
            current_version: record.task.version,
            event_id: Some(event_id),
        })
    }

    fn read_task(&self, root: &Path, task_id: TaskId) -> Result<Task, MinervaError> {
        task_repository_support::read_existing(&MinervaLayout::new(root), task_id)
    }

    fn update_task(
        &self,
        root: &Path,
        task: &Task,
    ) -> Result<TaskWriteResult, MinervaError> {
        let layout = MinervaLayout::new(root);
        let _lock = TaskLock::acquire(&layout, task.id)?;
        let previous = task_repository_support::read_existing(&layout, task.id)?;
        persist_task(&layout, task)?;
        Ok(TaskWriteResult {
            previous_version: Some(previous.version),
            current_version: task.version,
            event_id: None,
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
        let previous = task_repository_support::read_existing(&layout, task_id)?;
        let archived = task_repository_support::archive(previous.clone(), &layout, version)?;
        persist_task(&layout, &archived)?;
        Ok(TaskWriteResult {
            previous_version: Some(previous.version),
            current_version: archived.version,
            event_id: None,
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
