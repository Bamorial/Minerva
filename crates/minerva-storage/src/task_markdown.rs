use crate::atomic_replace;
use minerva_domain::MinervaError;
use std::{fs, path::Path};

pub fn read_required_markdown(path: &Path) -> Result<String, MinervaError> {
    fs::read_to_string(path).map_err(|err| missing_or_schema(path, err))
}

pub fn read_optional_markdown(path: &Path) -> Result<String, MinervaError> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(missing_or_schema(path, err)),
    }
}

pub fn write_markdown(path: &Path, contents: &str) -> Result<(), MinervaError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| schema(path, err))?;
    }
    atomic_replace(path, contents.as_bytes()).map_err(|err| schema(path, err))
}

fn missing_or_schema(path: &Path, err: std::io::Error) -> MinervaError {
    if err.kind() == std::io::ErrorKind::NotFound {
        return MinervaError::SchemaError {
            path: path.display().to_string(),
            reason: "required file is missing".into(),
        };
    }
    schema(path, err)
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
