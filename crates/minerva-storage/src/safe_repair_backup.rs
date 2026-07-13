use minerva_domain::MinervaError;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create(path: &Path, dry_run: bool) -> Result<PathBuf, MinervaError> {
    let backup = backup_path(path)?;
    if !dry_run {
        fs::copy(path, &backup).map_err(|err| schema(path, err))?;
    }
    Ok(backup)
}

fn backup_path(path: &Path) -> Result<PathBuf, MinervaError> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| schema(path, err))?
        .as_nanos();
    let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("repair");
    let name = name.trim_start_matches('.').replace(".tmp.", ".tmp-backup.");
    Ok(path.with_file_name(format!("safe-repair-backup-{stamp}-{name}")))
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
