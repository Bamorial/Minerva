#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppCommand {
    Reload,
    CreateTask { title: String },
    ChangeStatus { status: String },
    MoveTask { parent_ref: Option<String> },
    EditInstructions,
    EditDeclaration,
    AddDependency { depends_on_ref: String },
    RemoveDependency { depends_on_ref: String },
}
