use crate::{MinervaLayout, read_task, task_index, task_scan};
use minerva_domain::{MinervaError, Task, TaskId};

pub fn list_tasks(layout: &MinervaLayout) -> Result<Vec<Task>, MinervaError> {
    match task_index::read_indexed_tasks(layout)? {
        Some(tasks) => Ok(tasks),
        None => task_scan::scan_tasks(layout),
    }
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

fn eq(title: &str, query: &str) -> bool {
    title.trim().eq_ignore_ascii_case(query.trim())
}

fn contains(title: &str, query: &str) -> bool {
    title.to_lowercase().contains(&query.trim().to_lowercase())
}

fn label(task: Task) -> String {
    format!("{} {}", task.id, task.title)
}
