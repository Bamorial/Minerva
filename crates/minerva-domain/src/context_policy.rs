use crate::MinervaError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextPolicy {
    pub max_items: u16,
    pub max_dependency_hops: u8,
    pub stale_after_hours: u16,
}

impl ContextPolicy {
    pub fn new(
        max_items: u16,
        max_dependency_hops: u8,
        stale_after_hours: u16,
    ) -> Result<Self, MinervaError> {
        let policy = Self { max_items, max_dependency_hops, stale_after_hours };
        policy.validate()?;
        Ok(policy)
    }

    pub fn validate(&self) -> Result<(), MinervaError> {
        validate_non_zero("context_policy.max_items", self.max_items)?;
        validate_non_zero(
            "context_policy.max_dependency_hops",
            self.max_dependency_hops,
        )?;
        validate_non_zero("context_policy.stale_after_hours", self.stale_after_hours)
    }
}

fn validate_non_zero<T>(key: &str, value: T) -> Result<(), MinervaError>
where
    T: Default + PartialEq,
{
    if value == T::default() {
        return Err(MinervaError::InvalidConfiguration {
            key: key.into(),
            reason: "must be greater than zero".into(),
        });
    }
    Ok(())
}
