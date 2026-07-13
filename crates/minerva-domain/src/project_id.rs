use crate::IdentifierError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ulid::Ulid;

const PREFIX: &str = "PRJ-";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectId(Ulid);

impl ProjectId {
    #[must_use]
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

impl std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}{}", self.0)
    }
}

impl std::str::FromStr for ProjectId {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let raw = value.strip_prefix(PREFIX).ok_or(IdentifierError::InvalidPrefix {
            kind: "project",
            expected: PREFIX,
        })?;
        let parsed = raw.parse().map_err(|_| IdentifierError::InvalidBody {
            kind: "project",
            reason: "expected ULID body",
        })?;
        Ok(Self(parsed))
    }
}

impl Serialize for ProjectId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ProjectId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}
