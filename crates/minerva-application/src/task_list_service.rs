use crate::{
    ProjectRepository, TaskListItem, TaskListOptions, TaskListParent, TaskListResult,
    TaskListSort, TaskRepository,
};
use minerva_domain::{MinervaError, Task, TaskId, TaskPriority};
use std::{cmp::Reverse, collections::BTreeMap, path::Path};

pub struct TaskListService;

impl TaskListService {
    pub fn list(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        options: &TaskListOptions,
    ) -> Result<TaskListResult, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let all = task_repo.list_tasks(&root)?;
        let parent_id = options
            .parent_ref
            .as_deref()
            .map(|value| task_repo.resolve_task(&root, value).map(|task| task.id))
            .transpose()?;
        let map =
            all.iter().cloned().map(|task| (task.id, task)).collect::<BTreeMap<_, _>>();
        let total = all.len();
        let mut tasks = all
            .into_iter()
            .filter(|task| matches(task, options, parent_id))
            .collect::<Vec<_>>();
        sort(&mut tasks, options.sort);
        let matched = tasks.len();
        let slice = options.limit.map_or_else(
            || tasks.get(options.offset..).unwrap_or(&[]),
            |limit| {
                let end = options.offset.saturating_add(limit).min(matched);
                tasks.get(options.offset..end).unwrap_or(&[])
            },
        );
        Ok(TaskListResult {
            tasks: slice.iter().cloned().map(|task| item(&map, task)).collect(),
            total,
            matched,
            offset: options.offset,
            limit: options.limit,
            sort: options.sort,
            has_more: options.offset.saturating_add(slice.len()) < matched,
        })
    }
}

fn matches(task: &Task, options: &TaskListOptions, parent_id: Option<TaskId>) -> bool {
    options.status.as_ref().is_none_or(|value| task.status == *value)
        && options.task_type.as_ref().is_none_or(|value| task.task_type == *value)
        && parent_id.is_none_or(|value| task.parent_id == Some(value))
        && options.tag.as_ref().is_none_or(|value| task.tags.contains(value))
        && options.archive_state.matches(task.archive_state)
        && options.search.as_ref().is_none_or(|value| search(task, value))
}

fn search(task: &Task, query: &str) -> bool {
    let query = query.trim().to_ascii_lowercase();
    task.id.to_string().to_ascii_lowercase().contains(&query)
        || task.title.to_ascii_lowercase().contains(&query)
        || task.slug.as_ref().is_some_and(|value| value.as_str().contains(&query))
        || task.tags.iter().any(|value| value.as_str().contains(&query))
}

fn item(all: &BTreeMap<TaskId, Task>, task: Task) -> TaskListItem {
    let parent = task.parent_id.and_then(|id| {
        all.get(&id).map(|task| TaskListParent {
            id: task.id.to_string(),
            title: task.title.clone(),
        })
    });
    TaskListItem { task, parent }
}

fn sort(tasks: &mut [Task], sort: TaskListSort) {
    match sort {
        TaskListSort::Created => {
            tasks.sort_by_key(|task| (Reverse(task.created_at), task.id))
        }
        TaskListSort::Updated => {
            tasks.sort_by_key(|task| (Reverse(task.updated_at), task.id))
        }
        TaskListSort::Priority => {
            tasks.sort_by_key(|task| (Reverse(priority(task.priority)), task.id))
        }
        TaskListSort::Title => {
            tasks.sort_by_cached_key(|task| (task.title.to_ascii_lowercase(), task.id))
        }
        TaskListSort::Id => tasks.sort_by_key(|task| task.id),
    }
}

fn priority(value: TaskPriority) -> u8 {
    match value {
        TaskPriority::Low => 0,
        TaskPriority::Medium => 1,
        TaskPriority::High => 2,
        TaskPriority::Urgent => 3,
    }
}
