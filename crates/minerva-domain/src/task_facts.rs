use crate::{MinervaError, TaskResources};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskFacts {
    pub modules: Vec<String>,
    pub files: Vec<String>,
    pub migrations_required: bool,
    pub feature_flags: Vec<String>,
    pub acceptance_checks: Vec<String>,
    pub resources: TaskResources,
}

impl TaskFacts {
    pub fn validate(&self) -> Result<(), MinervaError> {
        validate_items("facts.modules", &self.modules)?;
        validate_items("facts.files", &self.files)?;
        validate_items("facts.feature_flags", &self.feature_flags)?;
        validate_items("facts.acceptance_checks", &self.acceptance_checks)?;
        self.resources.validate()
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
