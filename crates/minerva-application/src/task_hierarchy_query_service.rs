use crate::{TaskHierarchyQueryResult, TaskRepository};
use minerva_domain::{MinervaError, Task, TaskId, validate_task_hierarchy};
use std::{collections::BTreeMap, path::Path};

pub struct TaskHierarchyQueryService;

impl TaskHierarchyQueryService {
    pub fn children(
        task_repo: &impl TaskRepository,
        root: &Path,
        task_ref: &str,
    ) -> Result<TaskHierarchyQueryResult, MinervaError> {
        let task = task_repo.resolve_task(root, task_ref)?;
        let tasks = ordered(task_repo.list_tasks(root)?)?;
        Ok(TaskHierarchyQueryResult {
            items: tasks
                .values()
                .filter(|item| item.parent_id == Some(task.id))
                .cloned()
                .collect(),
            task,
        })
    }

    pub fn ancestors(
        task_repo: &impl TaskRepository,
        root: &Path,
        task_ref: &str,
    ) -> Result<TaskHierarchyQueryResult, MinervaError> {
        let task = task_repo.resolve_task(root, task_ref)?;
        let tasks = ordered(task_repo.list_tasks(root)?)?;
        let mut items = Vec::new();
        let mut current = task.parent_id;
        while let Some(parent_id) = current {
            let parent = tasks.get(&parent_id).ok_or_else(|| {
                MinervaError::TaskNotFound { task_ref: parent_id.to_string() }
            })?;
            items.push(parent.clone());
            current = parent.parent_id;
        }
        items.reverse();
        Ok(TaskHierarchyQueryResult { task, items })
    }
}

fn ordered(tasks: Vec<Task>) -> Result<BTreeMap<TaskId, Task>, MinervaError> {
    validate_task_hierarchy(&tasks)?;
    Ok(tasks.into_iter().map(|task| (task.id, task)).collect())
}
