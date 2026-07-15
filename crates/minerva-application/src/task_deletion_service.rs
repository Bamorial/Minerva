use crate::{ProjectRepository, TaskDeletionResult, TaskRepository};
use minerva_domain::{MinervaError, Task, TaskId};
use std::{collections::BTreeMap, path::Path};

pub struct TaskDeletionService;

impl TaskDeletionService {
    pub fn delete(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        task_ref: &str,
    ) -> Result<TaskDeletionResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let task = task_repo.resolve_task(&root, task_ref)?;
        let tasks = task_repo.list_tasks(&root)?;
        let deleted_task_ids = descendants(&tasks, task.id);
        task_repo.delete_tasks(&root, &deleted_task_ids)?;
        Ok(TaskDeletionResult { task, deleted_task_ids })
    }
}

fn descendants(tasks: &[Task], root_id: TaskId) -> Vec<TaskId> {
    let mut by_parent = BTreeMap::<Option<TaskId>, Vec<TaskId>>::new();
    for task in tasks {
        by_parent.entry(task.parent_id).or_default().push(task.id);
    }
    let mut deleted = Vec::new();
    let mut stack = vec![root_id];
    while let Some(task_id) = stack.pop() {
        deleted.push(task_id);
        if let Some(children) = by_parent.get(&Some(task_id)) {
            stack.extend(children.iter().rev().copied());
        }
    }
    deleted
}
