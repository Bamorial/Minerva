use crate::{MinervaLayout, read_task};
use minerva_domain::{MinervaError, Task, TaskId};
use std::fs;

pub fn scan_tasks(layout: &MinervaLayout) -> Result<Vec<Task>, MinervaError> {
    let mut tasks = task_ids(layout)?
        .into_iter()
        .map(|task_id| read_task(layout, task_id))
        .collect::<Result<Vec<_>, _>>()?;
    tasks.sort_by_key(|task| task.id);
    Ok(tasks)
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
        .collect();
    Ok(task_ids)
}

fn schema(layout: &MinervaLayout, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: layout.tasks_dir().display().to_string(),
        reason: err.to_string(),
    }
}
