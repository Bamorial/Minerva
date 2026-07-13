use crate::IdentifierError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ulid::Ulid;

const PREFIX: &str = "REL-";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelationshipId(Ulid);

impl RelationshipId {
    #[must_use]
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

impl std::fmt::Display for RelationshipId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX}{}", self.0)
    }
}

impl std::str::FromStr for RelationshipId {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let raw = value.strip_prefix(PREFIX).ok_or(IdentifierError::InvalidPrefix {
            kind: "relationship",
            expected: PREFIX,
        })?;
        let parsed = raw.parse().map_err(|_| IdentifierError::InvalidBody {
            kind: "relationship",
            reason: "expected ULID body",
        })?;
        Ok(Self(parsed))
    }
}

impl Serialize for RelationshipId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RelationshipId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}
