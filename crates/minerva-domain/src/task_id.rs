use crate::IdentifierError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicU32, Ordering};

const PREFIX: &str = "TSK-";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(NonZeroU32);

impl TaskId {
    #[must_use]
    pub const fn from_sequence(value: NonZeroU32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn sequence(self) -> NonZeroU32 {
        self.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}{:06}", self.0)
    }
}

impl std::str::FromStr for TaskId {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let raw = value
            .strip_prefix(PREFIX)
            .ok_or(IdentifierError::InvalidPrefix { kind: "task", expected: PREFIX })?;
        let parsed = raw.parse::<u32>().map_err(|_| IdentifierError::InvalidBody {
            kind: "task",
            reason: "expected digits",
        })?;
        let value = NonZeroU32::new(parsed).ok_or(IdentifierError::InvalidBody {
            kind: "task",
            reason: "sequence must be positive",
        })?;
        Ok(Self(value))
    }
}

impl Serialize for TaskId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TaskId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Default)]
pub struct TaskIdAllocator(AtomicU32);

impl TaskIdAllocator {
    #[must_use]
    pub const fn new(last_assigned: u32) -> Self {
        Self(AtomicU32::new(last_assigned))
    }

    #[must_use]
    pub fn next_id(&self) -> TaskId {
        let next = self.0.fetch_add(1, Ordering::Relaxed) + 1;
        TaskId::from_sequence(NonZeroU32::new(next).expect("task IDs start at 1"))
    }
}
