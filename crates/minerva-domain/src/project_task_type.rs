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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskTypeDefinition {
    pub name: TaskTypeKey,
    pub display_name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub declaration_requirements: Vec<String>,
    pub instruction_template: String,
}

impl TaskTypeDefinition {
    pub fn new(definition: Self) -> Result<Self, MinervaError> {
        definition.validate()?;
        Ok(definition)
    }

    pub fn validate(&self) -> Result<(), MinervaError> {
        if self.display_name.trim().is_empty() {
            return invalid("display_name", "must not be empty");
        }
        if self.instruction_template.trim().is_empty() {
            return invalid("instruction_template", "must not be empty");
        }
        if self.declaration_requirements.iter().any(|item| item.trim().is_empty()) {
            return invalid("declaration_requirements", "must not contain empty items");
        }
        Ok(())
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

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
