use crate::{MinervaLayout, schema_migration_v0_support};
use minerva_application::ProjectMigrationAction;
use minerva_domain::{MinervaError, TaskId};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct FilePlan {
    pub action: ProjectMigrationAction,
    pub path: PathBuf,
    pub contents: String,
    pub message: String,
}

pub fn planned_files(layout: &MinervaLayout) -> Result<Vec<FilePlan>, MinervaError> {
    let mut files = Vec::new();
    push(&mut files, file(layout.project_file(), "added schema version to project")?);
    push(&mut files, file(layout.config_file(), "added schema version to config")?);
    files.extend(tasks(layout)?);
    files.extend(relationships(layout)?);
    push(&mut files, schema_marker(layout));
    Ok(files)
}

pub fn relative(layout: &MinervaLayout, path: &Path) -> String {
    path.strip_prefix(layout.root()).unwrap_or(path).display().to_string()
}

pub fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}

fn push(files: &mut Vec<FilePlan>, item: Option<FilePlan>) {
    if let Some(file) = item {
        files.push(file);
    }
}

fn tasks(layout: &MinervaLayout) -> Result<Vec<FilePlan>, MinervaError> {
    schema_migration_v0_support::task_ids(layout)?
        .into_iter()
        .map(|task_id| file(layout.task_file(task_id), "added schema version to task"))
        .collect::<Result<Vec<_>, _>>()
        .map(|items| items.into_iter().flatten().collect())
}

fn relationships(layout: &MinervaLayout) -> Result<Vec<FilePlan>, MinervaError> {
    schema_migration_v0_support::task_ids(layout)?
        .into_iter()
        .map(|task_id| relationship_file(layout, task_id))
        .collect::<Result<Vec<_>, _>>()
        .map(|items| items.into_iter().flatten().collect())
}

fn relationship_file(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<Vec<FilePlan>, MinervaError> {
    let path = layout.relationships_file(task_id);
    if !path.exists() {
        return Ok(Vec::new());
    }
    schema_migration_v0_support::relationship_file(path)
        .map(|item| item.into_iter().collect())
}

fn file(path: PathBuf, message: &str) -> Result<Option<FilePlan>, MinervaError> {
    schema_migration_v0_support::file(path, message)
}

fn schema_marker(layout: &MinervaLayout) -> Option<FilePlan> {
    let path = layout.schema_version_file();
    let current = fs::read_to_string(&path).ok();
    (current.as_deref() != Some(crate::SCHEMA_VERSION)).then_some(FilePlan {
        action: ProjectMigrationAction::Create,
        path,
        contents: crate::SCHEMA_VERSION.into(),
        message: "created schema marker".into(),
    })
}
