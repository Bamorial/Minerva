use crate::project_document_parts::{
    ContextPolicyDocument, StatusDocument, TransitionDocument,
};
use humantime::{format_rfc3339, parse_rfc3339};
use minerva_domain::{MinervaError, Project, ProjectId};
use serde::{Deserialize, Serialize};

const SUPPORTED_SCHEMA: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDocument {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub default_task_type: String,
    pub default_status: String,
    pub statuses: Vec<StatusDocument>,
    pub transitions: Vec<TransitionDocument>,
    pub context_policy: ContextPolicyDocument,
}

impl TryFrom<ProjectDocument> for Project {
    type Error = MinervaError;

    fn try_from(doc: ProjectDocument) -> Result<Self, Self::Error> {
        require_schema(doc.schema_version)?;
        Project::new(Project {
            schema_version: doc.schema_version,
            id: doc.id.parse::<ProjectId>().map_err(|err| invalid(err.to_string()))?,
            name: doc.name,
            created_at: parse_rfc3339(&doc.created_at)
                .map_err(|err| invalid(err.to_string()))?,
            default_task_type: doc.default_task_type.parse()?,
            default_status: doc.default_status.parse()?,
            statuses: doc
                .statuses
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            transitions: doc
                .transitions
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            context_policy: doc.context_policy.try_into()?,
        })
    }
}

impl From<&Project> for ProjectDocument {
    fn from(project: &Project) -> Self {
        Self {
            schema_version: project.schema_version,
            id: project.id.to_string(),
            name: project.name.clone(),
            created_at: format_rfc3339(project.created_at).to_string(),
            default_task_type: project.default_task_type.as_str().into(),
            default_status: project.default_status.as_str().into(),
            statuses: project.statuses.iter().map(StatusDocument::from).collect(),
            transitions: project
                .transitions
                .iter()
                .map(TransitionDocument::from)
                .collect(),
            context_policy: (&project.context_policy).into(),
        }
    }
}

fn require_schema(schema_version: u32) -> Result<(), MinervaError> {
    if schema_version == SUPPORTED_SCHEMA {
        return Ok(());
    }
    Err(invalid(format!("unsupported schema version `{schema_version}`")))
}

fn invalid(reason: impl Into<String>) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: "schema_version".into(),
        reason: reason.into(),
    }
}
