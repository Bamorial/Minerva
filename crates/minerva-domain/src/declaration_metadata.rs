use crate::{DeclarationActor, MinervaError};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclarationMetadata {
    pub version: u32,
    pub updated_at: SystemTime,
    pub updated_by: DeclarationActor,
    pub commit_hash: Option<String>,
}

impl DeclarationMetadata {
    pub fn validate(&self) -> Result<(), MinervaError> {
        if self.version == 0 {
            return invalid("declaration.version", "must be greater than zero");
        }
        if matches!(self.commit_hash.as_deref(), Some("")) {
            return invalid("declaration.commit_hash", "must not be empty");
        }
        Ok(())
    }
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
