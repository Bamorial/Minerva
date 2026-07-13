use crate::MinervaLayout;
use minerva_application::{RepairAction, RepairKind, RepairOperation, RepairSafety};
use minerva_domain::MinervaError;
use std::fs;

pub fn repair(
    layout: &MinervaLayout,
    dry_run: bool,
) -> Result<Vec<RepairOperation>, MinervaError> {
    let mut operations = Vec::new();
    visit(&layout.minerva_dir(), &mut |path| {
        if !is_abandoned_temp(path) {
            return Ok(());
        }
        let backup = crate::safe_repair_backup::create(path, dry_run)?;
        if !dry_run {
            fs::remove_file(path).map_err(|err| schema(path, err))?;
        }
        operations.push(RepairOperation {
            kind: RepairKind::TemporaryFile,
            safety: RepairSafety::Safe,
            action: RepairAction::Remove,
            path: relative(layout, path),
            backup_path: Some(relative(layout, &backup)),
            message: "removed abandoned temporary file after creating backup".into(),
        });
        Ok(())
    })?;
    Ok(operations)
}

fn visit(
    path: &std::path::Path,
    handle: &mut impl FnMut(&std::path::Path) -> Result<(), MinervaError>,
) -> Result<(), MinervaError> {
    if !path.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(path).map_err(|err| schema(path, err))? {
        let entry = entry.map_err(|err| schema(path, err))?;
        let child = entry.path();
        if entry.file_type().map_err(|err| schema(&child, err))?.is_dir() {
            visit(&child, handle)?;
        } else {
            handle(&child)?;
        }
    }
    Ok(())
}

fn is_abandoned_temp(path: &std::path::Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    name.starts_with('.') && name.contains(".tmp.")
}

fn relative(layout: &MinervaLayout, path: &std::path::Path) -> String {
    path.strip_prefix(layout.root()).unwrap_or(path).display().to_string()
}

fn schema(path: &std::path::Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
