use crate::{MinervaLayout, schema_migration_v0};
use minerva_application::ProjectMigrationResult;
use minerva_domain::MinervaError;
use std::{fs, path::Path};

pub fn migrate_project_state(
    root: &Path,
    dry_run: bool,
) -> Result<ProjectMigrationResult, MinervaError> {
    let layout = MinervaLayout::new(root);
    let start_version = start_version(&layout)?;
    let mut steps = Vec::new();
    if start_version == 0 {
        if let Some(step) = schema_migration_v0::migrate(&layout, dry_run)? {
            steps.push(step);
        }
    }
    Ok(ProjectMigrationResult { start_version, target_version: 1, steps })
}

fn start_version(layout: &MinervaLayout) -> Result<u32, MinervaError> {
    let marker = layout.schema_version_file();
    if marker.exists() {
        let contents =
            fs::read_to_string(&marker).map_err(|err| schema(&marker, err))?;
        if contents != crate::SCHEMA_VERSION && !legacy_pending(layout)? {
            return Err(schema(
                &marker,
                format!("unsupported schema marker `{}`", contents.trim()),
            ));
        }
    }
    Ok(if legacy_pending(layout)? { 0 } else { 1 })
}

fn legacy_pending(layout: &MinervaLayout) -> Result<bool, MinervaError> {
    schema_migration_v0::migrate(layout, true).map(|step| step.is_some())
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
