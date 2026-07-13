use crate::{
    ArchiveState, DeclarationMetadata, MinervaError, StatusKey, TaskFacts, TaskId,
    TaskPriority, TaskSlug, TaskTag, TaskTypeKey, TaskVersion,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, time::SystemTime};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub schema_version: u32,
    pub id: TaskId,
    pub title: String,
    pub slug: Option<TaskSlug>,
    pub task_type: TaskTypeKey,
    pub status: StatusKey,
    pub parent_id: Option<TaskId>,
    pub priority: TaskPriority,
    pub tags: BTreeSet<TaskTag>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub completed_at: Option<SystemTime>,
    pub version: TaskVersion,
    pub declaration: DeclarationMetadata,
    pub facts: TaskFacts,
    pub archive_state: ArchiveState,
}

impl Task {
    pub fn new(task: Self) -> Result<Self, MinervaError> {
        task.validate()?;
        Ok(task)
    }

    pub fn validate(&self) -> Result<(), MinervaError> {
        if self.schema_version == 0 {
            return invalid("schema_version", "must be greater than zero");
        }
        if self.title.trim().is_empty() {
            return invalid("title", "must not be empty");
        }
        if self.parent_id == Some(self.id) {
            return invalid("parent_id", "task cannot be its own parent");
        }
        self.declaration.validate()?;
        self.facts.validate()?;
        if self.status.as_str() == "completed" && self.completed_at.is_none() {
            return invalid("completed_at", "is required when status is completed");
        }
        Ok(())
    }

    pub fn validate_successor(&self, previous: &Self) -> Result<(), MinervaError> {
        self.validate()?;
        if self.id != previous.id {
            return invalid("id", "must remain immutable across versions");
        }
        if self.version != previous.version.next() {
            return invalid("version", "must increase exactly by one");
        }
        Ok(())
    }
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
