use crate::MinervaError;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct TaskVersion(NonZeroU32);

impl TaskVersion {
    #[must_use]
    pub const fn initial() -> Self {
        Self(NonZeroU32::MIN)
    }

    pub fn new(value: u32) -> Result<Self, MinervaError> {
        NonZeroU32::new(value).map(Self).ok_or(MinervaError::InvalidConfiguration {
            key: "version".into(),
            reason: "must be greater than zero".into(),
        })
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }

    #[must_use]
    pub fn next(self) -> Self {
        Self(self.0.checked_add(1).expect("task version overflowed"))
    }
}
