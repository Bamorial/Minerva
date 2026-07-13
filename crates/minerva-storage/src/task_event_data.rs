use minerva_domain::{
    ArchiveState, DeclarationActor, Relationship, StatusKey, TaskId, TaskVersion,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventData {
    TaskCreated {
        version: TaskVersion,
        parent_id: Option<TaskId>,
        status: StatusKey,
    },
    TaskDeclarationUpdated {
        version: TaskVersion,
        declaration_version: u32,
        updated_by: DeclarationActor,
        commit_hash: Option<String>,
    },
    TaskInstructionsUpdated {
        version: TaskVersion,
    },
    TaskParentChanged {
        version: TaskVersion,
        from_parent_id: Option<TaskId>,
        to_parent_id: Option<TaskId>,
    },
    TaskStatusChanged {
        version: TaskVersion,
        from_status: StatusKey,
        to_status: StatusKey,
        completion_override: bool,
    },
    TaskRelationshipAdded {
        relationship: Relationship,
    },
    TaskRelationshipRemoved {
        relationship: Relationship,
    },
    TaskArchived {
        version: TaskVersion,
        from_archive_state: ArchiveState,
        to_archive_state: ArchiveState,
    },
}
