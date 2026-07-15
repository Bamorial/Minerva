use minerva_domain::{AgentPromptMode, MinervaError, ProjectConfig};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const SUPPORTED_SCHEMA: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectConfigDocument {
    pub schema_version: u32,
    pub editor: Option<String>,
    pub default_priority: minerva_domain::TaskPriority,
    pub default_tags: BTreeSet<minerva_domain::TaskTag>,
    #[serde(default = "default_prompt_mode")]
    pub agent_prompt_mode: String,
}

impl TryFrom<ProjectConfigDocument> for ProjectConfig {
    type Error = MinervaError;

    fn try_from(doc: ProjectConfigDocument) -> Result<Self, Self::Error> {
        require_schema(doc.schema_version)?;
        ProjectConfig::new(ProjectConfig {
            schema_version: doc.schema_version,
            editor: doc.editor,
            default_priority: doc.default_priority,
            default_tags: doc.default_tags,
            agent_prompt_mode: doc.agent_prompt_mode.parse()?,
        })
    }
}

impl From<&ProjectConfig> for ProjectConfigDocument {
    fn from(config: &ProjectConfig) -> Self {
        Self {
            schema_version: config.schema_version,
            editor: config.editor.clone(),
            default_priority: config.default_priority,
            default_tags: config.default_tags.clone(),
            agent_prompt_mode: config.agent_prompt_mode.to_string(),
        }
    }
}

fn default_prompt_mode() -> String {
    AgentPromptMode::Static.to_string()
}

fn require_schema(schema_version: u32) -> Result<(), MinervaError> {
    if schema_version == SUPPORTED_SCHEMA {
        return Ok(());
    }
    Err(MinervaError::InvalidConfiguration {
        key: "schema_version".into(),
        reason: format!("unsupported schema version `{schema_version}`"),
    })
}
