use crate::MinervaError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskResources {
    pub reads: Vec<String>,
    pub writes: Vec<String>,
}

impl TaskResources {
    pub fn validate(&self) -> Result<(), MinervaError> {
        validate_items("facts.resources.reads", &self.reads)?;
        validate_items("facts.resources.writes", &self.writes)
    }
}

fn validate_items(key: &str, items: &[String]) -> Result<(), MinervaError> {
    if items.iter().any(|item| item.trim().is_empty()) {
        return Err(MinervaError::InvalidConfiguration {
            key: key.into(),
            reason: "must not contain empty values".into(),
        });
    }
    Ok(())
}
