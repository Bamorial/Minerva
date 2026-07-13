use crate::MinervaError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusKey(String);

impl StatusKey {
    pub fn new(value: impl Into<String>) -> Result<Self, MinervaError> {
        let value = value.into();
        validate_key("status", &value)?;
        Ok(Self(value))
    }
}

impl std::fmt::Display for StatusKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusDefinition {
    pub key: StatusKey,
    pub terminal: bool,
}

impl StatusDefinition {
    #[must_use]
    pub const fn new(key: StatusKey, terminal: bool) -> Self {
        Self { key, terminal }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusTransition {
    pub from: StatusKey,
    pub to: StatusKey,
}

impl StatusTransition {
    #[must_use]
    pub const fn new(from: StatusKey, to: StatusKey) -> Self {
        Self { from, to }
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
