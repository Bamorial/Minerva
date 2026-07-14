use crate::{MinervaLayout, schema_migration_backup, schema_migration_v0_plan};
use minerva_application::{ProjectMigrationOperation, ProjectMigrationStep};
use minerva_domain::MinervaError;

pub const STEP_NAME: &str = "v0_to_v1";

pub fn migrate(
    layout: &MinervaLayout,
    dry_run: bool,
) -> Result<Option<ProjectMigrationStep>, MinervaError> {
    let files = schema_migration_v0_plan::planned_files(layout)?;
    if files.is_empty() {
        return Ok(None);
    }
    let mut operations = Vec::with_capacity(files.len());
    for file in files {
        let backup = schema_migration_backup::create(&file.path, dry_run)?;
        if !dry_run {
            crate::atomic_replace(&file.path, file.contents.as_bytes())
                .map_err(|err| schema_migration_v0_plan::schema(&file.path, err))?;
        }
        operations.push(ProjectMigrationOperation {
            action: file.action,
            path: schema_migration_v0_plan::relative(layout, &file.path),
            backup_path: backup
                .as_ref()
                .map(|path| schema_migration_v0_plan::relative(layout, path)),
            message: file.message,
        });
    }
    Ok(Some(ProjectMigrationStep {
        name: STEP_NAME.into(),
        from_version: 0,
        to_version: 1,
        operations,
    }))
}
