use crate::MinervaLayout;
use minerva_application::{RepairAction, RepairKind, RepairOperation, RepairSafety};
use minerva_domain::MinervaError;
use std::fs;

pub fn repair(
    layout: &MinervaLayout,
    dry_run: bool,
) -> Result<Vec<RepairOperation>, MinervaError> {
    let mut operations = Vec::new();
    for path in [
        layout.indexes_dir(),
        layout.contexts_dir(),
        layout.sessions_dir(),
        layout.locks_dir(),
    ] {
        if path.exists() {
            continue;
        }
        if !dry_run {
            fs::create_dir_all(&path).map_err(|err| schema(&path, err))?;
        }
        operations.push(RepairOperation {
            kind: RepairKind::LayoutDirectory,
            safety: RepairSafety::Safe,
            action: RepairAction::Create,
            path: relative(layout, &path),
            backup_path: None,
            message: "recreated required derived or operational directory".into(),
        });
    }
    Ok(operations)
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
