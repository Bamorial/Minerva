use crate::task_document_parts::{DeclarationMetadataDocument, TaskFactsDocument};
use humantime::{format_rfc3339, parse_rfc3339};
use minerva_domain::{MinervaError, Task};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const SUPPORTED_SCHEMA: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskDocument {
    pub schema_version: u32,
    pub id: minerva_domain::TaskId,
    pub title: String,
    pub slug: Option<minerva_domain::TaskSlug>,
    pub task_type: minerva_domain::TaskTypeKey,
    pub status: minerva_domain::StatusKey,
    pub parent_id: Option<minerva_domain::TaskId>,
    pub priority: minerva_domain::TaskPriority,
    pub tags: BTreeSet<minerva_domain::TaskTag>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
    pub version: minerva_domain::TaskVersion,
    pub declaration: DeclarationMetadataDocument,
    #[serde(default)]
    pub facts: TaskFactsDocument,
    pub archive_state: minerva_domain::ArchiveState,
}

impl TryFrom<TaskDocument> for Task {
    type Error = MinervaError;

    fn try_from(doc: TaskDocument) -> Result<Self, Self::Error> {
        require_schema(doc.schema_version)?;
        Task::new(Task {
            schema_version: doc.schema_version,
            id: doc.id,
            title: doc.title,
            slug: doc.slug,
            task_type: doc.task_type,
            status: doc.status,
            parent_id: doc.parent_id,
            priority: doc.priority,
            tags: doc.tags,
            created_at: parse_rfc3339(&doc.created_at)
                .map_err(|err| invalid(err.to_string()))?,
            updated_at: parse_rfc3339(&doc.updated_at)
                .map_err(|err| invalid(err.to_string()))?,
            completed_at: doc
                .completed_at
                .map(|value| parse_rfc3339(&value))
                .transpose()
                .map_err(|err| invalid(err.to_string()))?,
            version: doc.version,
            declaration: doc.declaration.try_into()?,
            facts: doc.facts.try_into()?,
            archive_state: doc.archive_state,
        })
    }
}

impl From<&Task> for TaskDocument {
    fn from(task: &Task) -> Self {
        Self {
            schema_version: task.schema_version,
            id: task.id,
            title: task.title.clone(),
            slug: task.slug.clone(),
            task_type: task.task_type.clone(),
            status: task.status.clone(),
            parent_id: task.parent_id,
            priority: task.priority,
            tags: task.tags.clone(),
            created_at: format_rfc3339(task.created_at).to_string(),
            updated_at: format_rfc3339(task.updated_at).to_string(),
            completed_at: task
                .completed_at
                .map(|value| format_rfc3339(value).to_string()),
            version: task.version,
            declaration: (&task.declaration).into(),
            facts: (&task.facts).into(),
            archive_state: task.archive_state,
        }
    }
}

fn require_schema(schema_version: u32) -> Result<(), MinervaError> {
    (schema_version == SUPPORTED_SCHEMA).then_some(()).ok_or_else(|| {
        invalid(format!("unsupported schema version `{schema_version}`"))
    })
}

fn invalid(reason: impl Into<String>) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: "schema_version".into(),
        reason: reason.into(),
    }
}
