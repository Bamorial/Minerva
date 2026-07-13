use minerva_domain::{MinervaError, StatusDefinition, StatusKey, StatusTransition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusDocument {
    pub key: String,
    pub terminal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionDocument {
    pub from: String,
    pub to: String,
}

impl TryFrom<StatusDocument> for StatusDefinition {
    type Error = MinervaError;

    fn try_from(doc: StatusDocument) -> Result<Self, Self::Error> {
        Ok(Self::new(StatusKey::new(doc.key)?, doc.terminal))
    }
}

impl From<&StatusDefinition> for StatusDocument {
    fn from(status: &StatusDefinition) -> Self {
        Self { key: status.key.as_str().into(), terminal: status.terminal }
    }
}

impl TryFrom<TransitionDocument> for StatusTransition {
    type Error = MinervaError;

    fn try_from(doc: TransitionDocument) -> Result<Self, Self::Error> {
        Ok(Self::new(doc.from.parse()?, doc.to.parse()?))
    }
}

impl From<&StatusTransition> for TransitionDocument {
    fn from(transition: &StatusTransition) -> Self {
        Self {
            from: transition.from.as_str().into(),
            to: transition.to.as_str().into(),
        }
    }
}
