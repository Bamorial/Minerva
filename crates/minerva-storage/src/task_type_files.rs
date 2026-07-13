use crate::{MinervaLayout, task_type_document::parse_task_type};
use minerva_domain::{MinervaError, TaskTypeDefinition};
use std::{collections::HashSet, fs};

pub fn read_task_types(
    layout: &MinervaLayout,
) -> Result<Vec<TaskTypeDefinition>, MinervaError> {
    let dir = layout.task_types_dir();
    let mut paths = fs::read_dir(&dir)
        .map_err(|err| io_schema(&dir, err))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| io_schema(&dir, err))?;
    paths.sort_by_key(|entry| entry.path());
    let mut seen = HashSet::new();
    let mut task_types = Vec::new();
    for entry in paths {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let source =
            path.file_name().and_then(|value| value.to_str()).unwrap_or("task-type");
        let contents =
            fs::read_to_string(&path).map_err(|err| io_schema(&path, err))?;
        let task_type =
            parse_task_type(source, &contents).map_err(|err| schema(&path, err))?;
        if !seen.insert(task_type.name.clone()) {
            return Err(schema(&path, duplicate(task_type.name.as_str())));
        }
        task_types.push(task_type);
    }
    Ok(task_types)
}

fn duplicate(name: &str) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: "name".into(),
        reason: format!("duplicate task type `{name}`"),
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

fn io_schema(path: &std::path::Path, err: std::io::Error) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
