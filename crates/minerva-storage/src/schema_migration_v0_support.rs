use crate::{
    schema_migration_v0_plan::{FilePlan, schema},
    schema_migration_value,
};
use minerva_application::ProjectMigrationAction;
use minerva_domain::{MinervaError, TaskId};
use serde_yaml::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn file(path: PathBuf, message: &str) -> Result<Option<FilePlan>, MinervaError> {
    let value = schema_migration_value::read(&path)?;
    let updated = Value::Mapping(migrate_mapping(&path, value)?);
    changed(path, updated, message)
}

pub fn relationship_file(path: PathBuf) -> Result<Option<FilePlan>, MinervaError> {
    let value = schema_migration_value::read(&path)?;
    let Value::Sequence(items) = value else {
        return Err(schema(&path, "expected relationship list"));
    };
    let updated: Vec<_> = items
        .into_iter()
        .map(|item| migrate_mapping(&path, item))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(Value::Mapping)
        .collect();
    changed(path, Value::Sequence(updated), "added schema version to relationships")
}

pub fn task_ids(layout: &crate::MinervaLayout) -> Result<Vec<TaskId>, MinervaError> {
    let mut task_ids: Vec<_> = fs::read_dir(layout.tasks_dir())
        .map_err(|err| schema(layout.tasks_dir().as_path(), err))?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry.file_type().ok().filter(std::fs::FileType::is_dir).map(|_| entry)
        })
        .filter_map(|entry| {
            entry.file_name().to_str().and_then(|value| value.parse().ok())
        })
        .collect();
    task_ids.sort_unstable();
    Ok(task_ids)
}

fn changed(
    path: PathBuf,
    value: Value,
    message: &str,
) -> Result<Option<FilePlan>, MinervaError> {
    let contents = serde_yaml::to_string(&value).map_err(|err| schema(&path, err))?;
    let current = fs::read_to_string(&path).map_err(|err| schema(&path, err))?;
    Ok((current != contents).then_some(FilePlan {
        action: ProjectMigrationAction::Update,
        path,
        contents,
        message: message.into(),
    }))
}

fn migrate_mapping(
    path: &Path,
    value: Value,
) -> Result<serde_yaml::Mapping, MinervaError> {
    let mapping = schema_migration_value::mapping(path, value)?;
    let key = Value::String("schema_version".into());
    match mapping.get(&key) {
        Some(Value::Number(value)) if value.as_u64() == Some(1) => Ok(mapping),
        Some(_) => Err(schema(path, "schema_version must be 1 when present")),
        None => Ok(schema_migration_value::prepend_schema_version(mapping)),
    }
}
