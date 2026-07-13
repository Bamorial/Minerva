use crate::{
    ProjectRepository, TaskRepository, TaskTreeNode, TaskTreeOptions, TaskTreeResult,
};
use minerva_domain::{MinervaError, Task, TaskId, validate_task_hierarchy};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

pub struct TaskTreeService;

impl TaskTreeService {
    pub fn tree(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        options: &TaskTreeOptions,
    ) -> Result<TaskTreeResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let tasks = task_repo.list_tasks(&root)?;
        validate_task_hierarchy(&tasks)?;
        let total = tasks.len();
        let matched = tasks
            .iter()
            .filter(|task| matches(task, options))
            .map(|task| task.id)
            .collect();
        let by_id = tasks.into_iter().map(|task| (task.id, task)).collect();
        let children = children(&by_id);
        Ok(TaskTreeResult {
            roots: nodes(None, &by_id, &children, &matched),
            total,
            matched: matched.len(),
        })
    }
}

fn matches(task: &Task, options: &TaskTreeOptions) -> bool {
    options.status.as_ref().is_none_or(|value| task.status == *value)
        && options.archive_state.matches(task.archive_state)
}

fn children(tasks: &BTreeMap<TaskId, Task>) -> BTreeMap<Option<TaskId>, Vec<TaskId>> {
    let mut children = BTreeMap::<Option<TaskId>, Vec<TaskId>>::new();
    for task in tasks.values() {
        children.entry(task.parent_id).or_default().push(task.id);
    }
    children
}

fn nodes(
    parent_id: Option<TaskId>,
    tasks: &BTreeMap<TaskId, Task>,
    children: &BTreeMap<Option<TaskId>, Vec<TaskId>>,
    matched: &BTreeSet<TaskId>,
) -> Vec<TaskTreeNode> {
    children
        .get(&parent_id)
        .into_iter()
        .flatten()
        .flat_map(|task_id| {
            let children = nodes(Some(*task_id), tasks, children, matched);
            if matched.contains(task_id) {
                vec![TaskTreeNode { task: tasks[task_id].clone(), children }]
            } else {
                children
            }
        })
        .collect()
}
