use crate::{
    MinervaLayout,
    task_document::TaskDocument,
    task_markdown::{read_optional_markdown, read_required_markdown, write_markdown},
    yaml_codec::{read_yaml, write_yaml},
};
use minerva_domain::{MinervaError, Task, TaskId};

pub fn read_task(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<Task, MinervaError> {
    let path = layout.task_file(task_id);
    if !path.exists() {
        return Err(MinervaError::SchemaError {
            path: path.display().to_string(),
            reason: "required file is missing".into(),
        });
    }
    read_yaml::<TaskDocument>(&path)?.try_into().map_err(|err| schema(&path, err))
}

pub fn write_task(layout: &MinervaLayout, task: &Task) -> Result<(), MinervaError> {
    let path = layout.task_file(task.id);
    task.validate().map_err(|err| schema(&path, err))?;
    if path.exists() {
        let previous = read_task(layout, task.id)?;
        if let Err(err) = task.validate_successor(&previous) {
            return Err(conflict_or_schema(&path, task, &previous, err));
        }
    }
    write_yaml(&path, &TaskDocument::from(task))
}

pub fn read_task_instructions(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<String, MinervaError> {
    read_optional_markdown(&layout.task_instructions_file(task_id))
}

pub fn write_task_instructions(
    layout: &MinervaLayout,
    task_id: TaskId,
    contents: &str,
) -> Result<(), MinervaError> {
    write_markdown(&layout.task_instructions_file(task_id), contents)
}

pub fn read_task_declaration(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<String, MinervaError> {
    read_required_markdown(&layout.declaration_file(task_id))
}

pub fn write_task_declaration(
    layout: &MinervaLayout,
    task_id: TaskId,
    contents: &str,
) -> Result<(), MinervaError> {
    write_markdown(&layout.declaration_file(task_id), contents)
}

pub fn read_task_notes(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<String, MinervaError> {
    read_optional_markdown(&layout.notes_file(task_id))
}

pub fn write_task_notes(
    layout: &MinervaLayout,
    task_id: TaskId,
    contents: &str,
) -> Result<(), MinervaError> {
    write_markdown(&layout.notes_file(task_id), contents)
}

fn conflict_or_schema(
    path: &std::path::Path,
    task: &Task,
    previous: &Task,
    err: MinervaError,
) -> MinervaError {
    match err {
        MinervaError::InvalidConfiguration { key, .. } if key == "version" => {
            MinervaError::VersionConflict {
                path: path.display().to_string(),
                expected: previous.version.next().get().to_string(),
                actual: task.version.get().to_string(),
            }
        }
        other => schema(path, other),
    }
}

fn schema(path: &std::path::Path, err: MinervaError) -> MinervaError {
    let reason = match err {
        MinervaError::InvalidConfiguration { key, reason } => {
            format!("{key}: {reason}")
        }
        MinervaError::SchemaError { reason, .. } => reason,
        other => other.to_string(),
    };
    MinervaError::SchemaError { path: path.display().to_string(), reason }
}
