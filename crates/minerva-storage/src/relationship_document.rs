use humantime::{format_rfc3339, parse_rfc3339};
use minerva_domain::{MinervaError, Relationship};
use serde::{Deserialize, Serialize};

const SUPPORTED_SCHEMA: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationshipDocument {
    pub schema_version: u32,
    pub id: minerva_domain::RelationshipId,
    pub source_task: minerva_domain::TaskId,
    pub target_task: minerva_domain::TaskId,
    pub relationship_type: minerva_domain::RelationshipType,
    pub reason: Option<String>,
    pub created_at: String,
}

impl TryFrom<RelationshipDocument> for Relationship {
    type Error = MinervaError;

    fn try_from(doc: RelationshipDocument) -> Result<Self, Self::Error> {
        require_schema(doc.schema_version)?;
        Relationship::new(Relationship {
            schema_version: doc.schema_version,
            id: doc.id,
            source_task: doc.source_task,
            target_task: doc.target_task,
            relationship_type: doc.relationship_type,
            reason: doc.reason,
            created_at: parse_rfc3339(&doc.created_at)
                .map_err(|err| invalid(err.to_string()))?,
        })
    }
}

impl From<&Relationship> for RelationshipDocument {
    fn from(value: &Relationship) -> Self {
        Self {
            schema_version: value.schema_version,
            id: value.id,
            source_task: value.source_task,
            target_task: value.target_task,
            relationship_type: value.relationship_type,
            reason: value.reason.clone(),
            created_at: format_rfc3339(value.created_at).to_string(),
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
