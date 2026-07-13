use minerva_domain::DeclarationActor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum TaskEventActor {
    Human,
    System,
    Agent { name: String },
}

impl From<DeclarationActor> for TaskEventActor {
    fn from(value: DeclarationActor) -> Self {
        match value {
            DeclarationActor::Human => Self::Human,
            DeclarationActor::System => Self::System,
            DeclarationActor::Agent(name) => Self::Agent { name },
        }
    }
}
