use crate::{
    MinervaLayout, TaskLock, append_created_event, append_moved_event,
    create_relationship as persist_relationship,
    remove_relationship as delete_relationship, task_hierarchy,
    task_repository_support, write_task as persist_task, write_task_declaration,
    write_task_instructions, write_task_notes,
};
use minerva_application::{TaskCreateRecord, TaskWriteResult};
use minerva_domain::{
    MinervaError, Relationship, RelationshipId, Task, TaskId, TaskVersion,
};
use std::{path::Path, time::SystemTime};

pub fn create_task(
    root: &Path,
    record: &TaskCreateRecord,
) -> Result<TaskWriteResult, MinervaError> {
    let layout = MinervaLayout::new(root);
    let _lock = TaskLock::acquire(&layout, record.task.id)?;
    task_hierarchy::validate_write(&layout, &record.task)?;
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

pub fn update_task(root: &Path, task: &Task) -> Result<TaskWriteResult, MinervaError> {
    let layout = MinervaLayout::new(root);
    let _lock = TaskLock::acquire(&layout, task.id)?;
    let previous = task_repository_support::read_existing(&layout, task.id)?;
    task_hierarchy::validate_write(&layout, task)?;
    persist_task(&layout, task)?;
    Ok(TaskWriteResult {
        previous_version: Some(previous.version),
        current_version: task.version,
        event_id: None,
    })
}

pub fn archive_task(
    root: &Path,
    task_id: TaskId,
    version: TaskVersion,
) -> Result<TaskWriteResult, MinervaError> {
    let layout = MinervaLayout::new(root);
    let _lock = TaskLock::acquire(&layout, task_id)?;
    let previous = task_repository_support::read_existing(&layout, task_id)?;
    let archived =
        task_repository_support::archive(previous.clone(), &layout, version)?;
    persist_task(&layout, &archived)?;
    Ok(TaskWriteResult {
        previous_version: Some(previous.version),
        current_version: archived.version,
        event_id: None,
    })
}

pub fn move_task(
    root: &Path,
    task_id: TaskId,
    new_parent_id: Option<TaskId>,
    version: TaskVersion,
) -> Result<(Task, TaskWriteResult), MinervaError> {
    let layout = MinervaLayout::new(root);
    let _lock = TaskLock::acquire(&layout, task_id)?;
    if let Some(parent_id) = new_parent_id {
        task_repository_support::read_existing(&layout, parent_id)?;
    }
    let previous = task_repository_support::read_existing(&layout, task_id)?;
    if previous.version != version {
        return Err(MinervaError::VersionConflict {
            path: layout.task_file(task_id).display().to_string(),
            expected: previous.version.get().to_string(),
            actual: version.get().to_string(),
        });
    }
    let moved = Task {
        parent_id: new_parent_id,
        updated_at: SystemTime::now(),
        version: previous.version.next(),
        ..previous.clone()
    };
    task_hierarchy::validate_write(&layout, &moved)?;
    persist_task(&layout, &moved)?;
    let event_id = append_moved_event(&layout, &moved, previous.parent_id)?;
    let result = TaskWriteResult {
        previous_version: Some(previous.version),
        current_version: moved.version,
        event_id: Some(event_id),
    };
    Ok((moved, result))
}

pub fn create_relationship(
    root: &Path,
    relationship: &Relationship,
) -> Result<Relationship, MinervaError> {
    persist_relationship(&MinervaLayout::new(root), relationship)
}

pub fn remove_relationship(
    root: &Path,
    relationship_id: RelationshipId,
) -> Result<Relationship, MinervaError> {
    delete_relationship(&MinervaLayout::new(root), relationship_id)
}
