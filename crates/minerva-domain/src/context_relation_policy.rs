use crate::MinervaError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextRelationPolicy {
    pub detail: crate::ContextDetail,
    pub depth: u8,
}

impl ContextRelationPolicy {
    pub fn validate(&self, key: &str) -> Result<(), MinervaError> {
        if self.depth > 0 {
            return Ok(());
        }
        Err(MinervaError::InvalidConfiguration {
            key: format!("context_policy.{key}.depth"),
            reason: "must be greater than zero".into(),
        })
    }
}
