use minerva_domain::MinervaError;
use std::path::{Path, PathBuf};

pub fn create(path: &Path, dry_run: bool) -> Result<Option<PathBuf>, MinervaError> {
    if !path.exists() {
        return Ok(None);
    }
    let backup = backup_path(path)?;
    if !dry_run {
        std::fs::copy(path, &backup).map_err(|err| schema(path, err))?;
    }
    Ok(Some(backup))
}

fn backup_path(path: &Path) -> Result<PathBuf, MinervaError> {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|err| schema(path, err))?
        .as_secs();
    let name = path.file_name().and_then(|value| value.to_str()).unwrap_or("file");
    Ok(path.with_file_name(format!("schema-migration-backup-{stamp}-{name}")))
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
