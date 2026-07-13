use crate::{MinervaLayout, read_task};
use minerva_domain::{MinervaError, Task, TaskId};
use std::fs;

pub fn list_tasks(layout: &MinervaLayout) -> Result<Vec<Task>, MinervaError> {
    let mut tasks = task_ids(layout)?
        .into_iter()
        .map(|task_id| read_task(layout, task_id))
        .collect::<Result<Vec<_>, _>>()?;
    tasks.sort_by_key(|task| task.id);
    Ok(tasks)
}

pub fn search_tasks(
    layout: &MinervaLayout,
    query: &str,
) -> Result<Vec<Task>, MinervaError> {
    let tasks = list_tasks(layout)?;
    let exact: Vec<_> =
        tasks.iter().filter(|task| eq(&task.title, query)).cloned().collect();
    if !exact.is_empty() {
        return Ok(exact);
    }
    Ok(tasks.into_iter().filter(|task| contains(&task.title, query)).collect())
}

pub fn resolve_task(
    layout: &MinervaLayout,
    task_ref: &str,
) -> Result<Task, MinervaError> {
    if let Ok(task_id) = task_ref.parse::<TaskId>() {
        return layout
            .task_file(task_id)
            .exists()
            .then(|| read_task(layout, task_id))
            .unwrap_or_else(|| {
                Err(MinervaError::TaskNotFound { task_ref: task_ref.into() })
            });
    }
    let matches = search_tasks(layout, task_ref)?;
    match matches.as_slice() {
        [] => Err(MinervaError::TaskNotFound { task_ref: task_ref.into() }),
        [task] => Ok(task.clone()),
        _ => Err(MinervaError::AmbiguousTaskReference {
            task_ref: task_ref.into(),
            matches: matches.into_iter().map(label).collect(),
        }),
    }
}

fn task_ids(layout: &MinervaLayout) -> Result<Vec<TaskId>, MinervaError> {
    if !layout.tasks_dir().exists() {
        return Ok(Vec::new());
    }
    let task_ids = fs::read_dir(layout.tasks_dir())
        .map_err(|err| schema(layout, err))?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry.file_type().ok().filter(|kind| kind.is_dir()).map(|_| entry)
        })
        .filter_map(|entry| {
            entry.file_name().to_str().and_then(|value| value.parse().ok())
        })
        .collect::<Vec<_>>();
    Ok(task_ids)
}

fn eq(title: &str, query: &str) -> bool {
    title.trim().eq_ignore_ascii_case(query.trim())
}

fn contains(title: &str, query: &str) -> bool {
    title.to_lowercase().contains(&query.trim().to_lowercase())
}

fn label(task: Task) -> String {
    format!("{} {}", task.id, task.title)
}

fn schema(layout: &MinervaLayout, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: layout.tasks_dir().display().to_string(),
        reason: err.to_string(),
    }
}
