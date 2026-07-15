use minerva_domain::AgentPromptMode;
use minerva_domain::RelationshipType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppCommand {
    Reload,
    CreateTask {
        title: String,
        task_type: String,
        parent_id: Option<minerva_domain::TaskId>,
    },
    ChangeStatus {
        status: String,
    },
    MoveTask {
        parent_ref: Option<String>,
    },
    EditInstructions,
    EditProjectInstructions,
    ShowContext {
        mode: AgentPromptMode,
    },
    SetPromptMode {
        mode: AgentPromptMode,
    },
    CopyContext,
    AddRelationship {
        task_ref: String,
        relationship_type: RelationshipType,
    },
    RemoveDependency {
        depends_on_ref: String,
    },
    DeleteTask {
        task_ref: String,
    },
}
