use crate::{MinervaLayout, TaskIndexDocument, atomic_replace, task_scan};
use minerva_domain::{MinervaError, Task};
use std::{fs, path::Path, time::UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskIndexStatus {
    Missing,
    Fresh,
    Stale,
}

pub fn refresh_task_index(layout: &MinervaLayout) -> Result<(), MinervaError> {
    let path = layout.task_index_file();
    let bytes = serde_json::to_vec_pretty(&TaskIndexDocument::from_tasks(
        &task_scan::scan_tasks(layout)?,
    ))
    .map_err(|err| schema(&path, err))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| schema(&path, err))?;
    }
    atomic_replace(&path, &bytes).map_err(|err| schema(&path, err))
}

pub fn read_indexed_tasks(
    layout: &MinervaLayout,
) -> Result<Option<Vec<Task>>, MinervaError> {
    let Some(doc) = read_task_index(layout)? else { return Ok(None) };
    Ok((!index_is_stale(layout, doc.tasks.len())?).then_some(doc.tasks))
}

pub fn task_index_status(
    layout: &MinervaLayout,
) -> Result<TaskIndexStatus, MinervaError> {
    let Some(doc) = read_task_index(layout)? else {
        return Ok(TaskIndexStatus::Missing);
    };
    Ok(if index_is_stale(layout, doc.tasks.len())? {
        TaskIndexStatus::Stale
    } else {
        TaskIndexStatus::Fresh
    })
}

fn read_task_index(
    layout: &MinervaLayout,
) -> Result<Option<TaskIndexDocument>, MinervaError> {
    let path = layout.task_index_file();
    if !path.exists() {
        return Ok(None);
    }
    let contents = fs::read_to_string(&path).map_err(|err| schema(&path, err))?;
    let doc: TaskIndexDocument =
        serde_json::from_str(&contents).map_err(|err| schema(&path, err))?;
    doc.validate().map_err(|err| schema(&path, err))?;
    Ok(Some(doc))
}

fn index_is_stale(
    layout: &MinervaLayout,
    expected: usize,
) -> Result<bool, MinervaError> {
    let path = layout.task_index_file();
    let index_time = fs::metadata(&path)
        .and_then(|meta| meta.modified())
        .map_err(|err| schema(&path, err))?;
    let mut count = 0;
    let mut newest = UNIX_EPOCH;
    for entry in fs::read_dir(layout.tasks_dir())
        .map_err(|err| schema(layout.tasks_dir().as_path(), err))?
    {
        let entry = entry.map_err(|err| schema(layout.tasks_dir().as_path(), err))?;
        let task_file = entry.path().join("task.yaml");
        if task_file.exists() {
            count += 1;
            newest = newest.max(
                fs::metadata(&task_file)
                    .and_then(|meta| meta.modified())
                    .map_err(|err| schema(&task_file, err))?,
            );
        }
    }
    Ok(count != expected || newest > index_time)
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
