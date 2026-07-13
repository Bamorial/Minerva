use humantime::{format_rfc3339, parse_rfc3339};
use minerva_domain::{DeclarationMetadata, MinervaError, TaskFacts, TaskResources};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationMetadataDocument {
    pub version: u32,
    pub updated_at: String,
    pub updated_by: minerva_domain::DeclarationActor,
    pub commit_hash: Option<String>,
}

impl TryFrom<DeclarationMetadataDocument> for DeclarationMetadata {
    type Error = MinervaError;

    fn try_from(doc: DeclarationMetadataDocument) -> Result<Self, Self::Error> {
        Ok(Self {
            version: doc.version,
            updated_at: parse_rfc3339(&doc.updated_at)
                .map_err(|err| invalid(err.to_string()))?,
            updated_by: doc.updated_by,
            commit_hash: doc.commit_hash,
        })
    }
}

impl From<&DeclarationMetadata> for DeclarationMetadataDocument {
    fn from(metadata: &DeclarationMetadata) -> Self {
        Self {
            version: metadata.version,
            updated_at: format_rfc3339(metadata.updated_at).to_string(),
            updated_by: metadata.updated_by.clone(),
            commit_hash: metadata.commit_hash.clone(),
        }
    }
}

fn invalid(reason: impl Into<String>) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: "declaration.updated_at".into(),
        reason: reason.into(),
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TaskFactsDocument {
    pub modules: Vec<String>,
    pub files: Vec<String>,
    pub migrations_required: bool,
    pub feature_flags: Vec<String>,
    pub acceptance_checks: Vec<String>,
    pub resources: TaskResourcesDocument,
}

impl TryFrom<TaskFactsDocument> for TaskFacts {
    type Error = MinervaError;

    fn try_from(doc: TaskFactsDocument) -> Result<Self, Self::Error> {
        Ok(Self {
            modules: doc.modules,
            files: doc.files,
            migrations_required: doc.migrations_required,
            feature_flags: doc.feature_flags,
            acceptance_checks: doc.acceptance_checks,
            resources: doc.resources.into(),
        })
    }
}

impl From<&TaskFacts> for TaskFactsDocument {
    fn from(facts: &TaskFacts) -> Self {
        Self {
            modules: facts.modules.clone(),
            files: facts.files.clone(),
            migrations_required: facts.migrations_required,
            feature_flags: facts.feature_flags.clone(),
            acceptance_checks: facts.acceptance_checks.clone(),
            resources: (&facts.resources).into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TaskResourcesDocument {
    pub reads: Vec<String>,
    pub writes: Vec<String>,
}

impl From<TaskResourcesDocument> for TaskResources {
    fn from(doc: TaskResourcesDocument) -> Self {
        Self { reads: doc.reads, writes: doc.writes }
    }
}

impl From<&TaskResources> for TaskResourcesDocument {
    fn from(resources: &TaskResources) -> Self {
        Self { reads: resources.reads.clone(), writes: resources.writes.clone() }
    }
}
