use crate::MinervaError;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AgentPromptMode {
    #[default]
    Static,
    Exploration,
}

impl AgentPromptMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Exploration => "exploration",
        }
    }
}

impl fmt::Display for AgentPromptMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for AgentPromptMode {
    type Err = MinervaError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "static" => Ok(Self::Static),
            "exploration" => Ok(Self::Exploration),
            other => Err(MinervaError::InvalidConfiguration {
                key: "agent_prompt_mode".into(),
                reason: format!("unsupported mode `{other}`"),
            }),
        }
    }
}
