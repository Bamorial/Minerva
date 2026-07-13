use crate::MinervaError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclarationActor {
    Human,
    System,
    Agent(String),
}

impl DeclarationActor {
    pub fn agent(name: impl Into<String>) -> Result<Self, MinervaError> {
        let name = name.into();
        if name.trim().is_empty() {
            return invalid("declaration.updated_by", "agent name must not be empty");
        }
        Ok(Self::Agent(name))
    }
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
