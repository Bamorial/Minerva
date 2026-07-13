use crate::MinervaError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskTypeKey(String);

impl TaskTypeKey {
    pub fn new(value: impl Into<String>) -> Result<Self, MinervaError> {
        let value = value.into();
        validate_key("default_task_type", &value)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TaskTypeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for TaskTypeKey {
    type Err = MinervaError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

fn validate_key(key: &str, value: &str) -> Result<(), MinervaError> {
    let valid = !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        });
    if valid {
        return Ok(());
    }
    Err(MinervaError::InvalidConfiguration {
        key: key.into(),
        reason: "must use lowercase letters, digits, or hyphens".into(),
    })
}
