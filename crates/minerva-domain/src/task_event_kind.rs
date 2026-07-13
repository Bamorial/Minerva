use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskEventKind {
    TaskCreated,
    TaskDeclarationUpdated,
    TaskInstructionsUpdated,
    TaskParentChanged,
    TaskStatusChanged,
    TaskRelationshipAdded,
    TaskRelationshipRemoved,
    TaskArchived,
}

impl TaskEventKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskCreated => "task-created",
            Self::TaskDeclarationUpdated => "task-declaration-updated",
            Self::TaskInstructionsUpdated => "task-instructions-updated",
            Self::TaskParentChanged => "task-parent-changed",
            Self::TaskStatusChanged => "task-status-changed",
            Self::TaskRelationshipAdded => "task-relationship-added",
            Self::TaskRelationshipRemoved => "task-relationship-removed",
            Self::TaskArchived => "task-archived",
        }
    }
}
