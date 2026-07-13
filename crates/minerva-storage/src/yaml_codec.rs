use crate::atomic_replace;
use serde::{Serialize, de::DeserializeOwned};
use std::{fs, path::Path};

pub fn read_yaml<T: DeserializeOwned>(
    path: &Path,
) -> Result<T, minerva_domain::MinervaError> {
    let contents = fs::read_to_string(path).map_err(|err| schema(path, err))?;
    serde_yaml::from_str(&contents).map_err(|err| schema(path, err))
}

pub fn write_yaml<T: Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), minerva_domain::MinervaError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| schema(path, err))?;
    }
    let contents = serde_yaml::to_string(value).map_err(|err| schema(path, err))?;
    atomic_replace(path, contents.as_bytes()).map_err(|err| schema(path, err))
}

fn schema(path: &Path, err: impl std::fmt::Display) -> minerva_domain::MinervaError {
    minerva_domain::MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
