use crate::MinervaError;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct TaskTag(String);

impl TaskTag {
    pub fn new(value: impl Into<String>) -> Result<Self, MinervaError> {
        let value = value.into();
        let valid = !value.is_empty()
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
            });
        if valid {
            return Ok(Self(value));
        }
        Err(MinervaError::InvalidConfiguration {
            key: "tags".into(),
            reason: "must use lowercase letters, digits, or hyphens".into(),
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
