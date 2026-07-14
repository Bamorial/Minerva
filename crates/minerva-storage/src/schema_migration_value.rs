use minerva_domain::MinervaError;
use serde_yaml::{Mapping, Value};
use std::{fs, path::Path};

pub fn read(path: &Path) -> Result<Value, MinervaError> {
    fs::read_to_string(path).map_err(|err| schema(path, err)).and_then(|contents| {
        serde_yaml::from_str(&contents).map_err(|err| schema(path, err))
    })
}

pub fn mapping(path: &Path, value: Value) -> Result<Mapping, MinervaError> {
    match value {
        Value::Mapping(mapping) => Ok(mapping),
        _ => Err(schema(path, "expected mapping document")),
    }
}

pub fn prepend_schema_version(mapping: Mapping) -> Mapping {
    let mut updated = Mapping::new();
    updated.insert(Value::String("schema_version".into()), Value::Number(1.into()));
    updated.extend(mapping);
    updated
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
