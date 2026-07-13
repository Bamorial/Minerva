use crate::MinervaError;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct TaskSlug(String);

impl TaskSlug {
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
            key: "slug".into(),
            reason: "must use lowercase letters, digits, or hyphens".into(),
        })
    }
}
