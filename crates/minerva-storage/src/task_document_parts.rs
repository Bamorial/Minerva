use humantime::{format_rfc3339, parse_rfc3339};
use minerva_domain::{DeclarationMetadata, MinervaError};
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
