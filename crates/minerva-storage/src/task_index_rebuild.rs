use crate::{MinervaLayout, TaskIndexDocument, atomic_replace, read_task};
use minerva_application::{RebuildAction, RebuildResult, RebuildTaskError};
use minerva_domain::{MinervaError, Task, TaskId};
use std::{fs, path::Path};

pub fn rebuild_task_index(
    layout: &MinervaLayout,
    dry_run: bool,
) -> Result<RebuildResult, MinervaError> {
    let (tasks, task_errors) = scan_valid_tasks(layout)?;
    let path = layout.task_index_file();
    let bytes = serde_json::to_vec_pretty(&TaskIndexDocument::from_tasks(&tasks))
        .map_err(|err| schema(&path, err))?;
    let index_action = compare(&path, &bytes)?;
    if !dry_run && index_action != RebuildAction::NoChange {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| schema(&path, err))?;
        }
        atomic_replace(&path, &bytes).map_err(|err| schema(&path, err))?;
    }
    Ok(RebuildResult { index_path: relative(layout, &path), index_action, task_errors })
}

fn scan_valid_tasks(
    layout: &MinervaLayout,
) -> Result<(Vec<Task>, Vec<RebuildTaskError>), MinervaError> {
    if !layout.tasks_dir().exists() {
        return Ok((Vec::new(), Vec::new()));
    }
    let mut tasks = Vec::new();
    let mut task_errors = Vec::new();
    for entry in fs::read_dir(layout.tasks_dir())
        .map_err(|err| schema_path(layout.tasks_dir().as_path(), err))?
    {
        let entry =
            entry.map_err(|err| schema_path(layout.tasks_dir().as_path(), err))?;
        if !entry.file_type().map_err(|err| schema_path(&entry.path(), err))?.is_dir() {
            continue;
        }
        let task_file = entry.path().join("task.yaml");
        if !task_file.exists() {
            continue;
        }
        match entry.file_name().to_string_lossy().parse::<TaskId>() {
            Ok(task_id) => match read_task(layout, task_id) {
                Ok(task) => tasks.push(task),
                Err(err) => {
                    task_errors.push(task_error(
                        layout,
                        &task_id.to_string(),
                        &task_file,
                        err,
                    ));
                }
            },
            Err(_) => task_errors.push(RebuildTaskError {
                task_ref: entry.file_name().to_string_lossy().into_owned(),
                path: relative(layout, &task_file),
                reason: "directory name is not a valid task id".into(),
            }),
        }
    }
    tasks.sort_by_key(|task| task.id);
    Ok((tasks, task_errors))
}

fn compare(path: &Path, bytes: &[u8]) -> Result<RebuildAction, MinervaError> {
    if !path.exists() {
        return Ok(RebuildAction::Create);
    }
    Ok(if fs::read(path).map_err(|err| schema(path, err))? == bytes {
        RebuildAction::NoChange
    } else {
        RebuildAction::Update
    })
}

fn task_error(
    layout: &MinervaLayout,
    task_ref: &str,
    path: &Path,
    err: MinervaError,
) -> RebuildTaskError {
    RebuildTaskError {
        task_ref: task_ref.into(),
        path: relative(layout, path),
        reason: match err {
            MinervaError::SchemaError { reason, .. }
            | MinervaError::InvalidConfiguration { reason, .. } => reason,
            other => other.to_string(),
        },
    }
}

fn relative(layout: &MinervaLayout, path: &Path) -> String {
    path.strip_prefix(layout.root()).unwrap_or(path).display().to_string()
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}

fn schema_path(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
