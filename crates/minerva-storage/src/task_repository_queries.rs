use crate::{
    MinervaLayout, list_relationships as load_relationships,
    list_relationships_from as load_relationships_from,
    list_relationships_to as load_relationships_to,
    read_task_instructions as load_task_instructions, task_repository_support,
};
use minerva_domain::{MinervaError, Relationship, Task, TaskId, TaskIdAllocator};
use std::path::Path;

pub fn next_task_id(root: &Path) -> Result<TaskId, MinervaError> {
    let last = list_tasks(root)?.into_iter().map(|task| task.id.sequence().get()).max();
    Ok(TaskIdAllocator::new(last.unwrap_or(0)).next_id())
}

pub fn read_task(root: &Path, task_id: TaskId) -> Result<Task, MinervaError> {
    task_repository_support::read_existing(&MinervaLayout::new(root), task_id)
}

pub fn read_task_instructions(
    root: &Path,
    task_id: TaskId,
) -> Result<String, MinervaError> {
    load_task_instructions(&MinervaLayout::new(root), task_id)
}

pub fn list_tasks(root: &Path) -> Result<Vec<Task>, MinervaError> {
    crate::task_catalog::list_tasks(&MinervaLayout::new(root))
}

pub fn list_relationships(root: &Path) -> Result<Vec<Relationship>, MinervaError> {
    load_relationships(&MinervaLayout::new(root))
}

pub fn list_relationships_from(
    root: &Path,
    task_id: TaskId,
) -> Result<Vec<Relationship>, MinervaError> {
    load_relationships_from(&MinervaLayout::new(root), task_id)
}

pub fn list_relationships_to(
    root: &Path,
    task_id: TaskId,
) -> Result<Vec<Relationship>, MinervaError> {
    load_relationships_to(&MinervaLayout::new(root), task_id)
}

pub fn resolve_task(root: &Path, task_ref: &str) -> Result<Task, MinervaError> {
    crate::task_catalog::resolve_task(&MinervaLayout::new(root), task_ref)
}

pub fn search_tasks(root: &Path, query: &str) -> Result<Vec<Task>, MinervaError> {
    crate::task_catalog::search_tasks(&MinervaLayout::new(root), query)
}
