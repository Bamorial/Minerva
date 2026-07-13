use crate::{MinervaError, RelationshipId, RelationshipType, TaskId};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Relationship {
    pub schema_version: u32,
    pub id: RelationshipId,
    pub source_task: TaskId,
    pub target_task: TaskId,
    pub relationship_type: RelationshipType,
    pub reason: Option<String>,
    pub created_at: SystemTime,
}

impl Relationship {
    pub fn new(relationship: Self) -> Result<Self, MinervaError> {
        relationship.validate()?;
        Ok(relationship)
    }

    pub fn validate(&self) -> Result<(), MinervaError> {
        if self.schema_version == 0 {
            return invalid("schema_version", "must be greater than zero");
        }
        if self.source_task == self.target_task {
            return invalid(
                "target_task",
                "must differ from source_task for this relationship type",
            );
        }
        if let Some(reason) = &self.reason
            && reason.trim().is_empty()
        {
            return invalid("reason", "must not be empty when present");
        }
        Ok(())
    }

    #[must_use]
    pub fn semantic_key(&self) -> (TaskId, TaskId, RelationshipType) {
        self.relationship_type.semantic_key(self.source_task, self.target_task)
    }
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
