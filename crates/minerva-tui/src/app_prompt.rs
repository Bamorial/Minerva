#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptKind {
    CreateTask,
    MoveTask,
    AddDependency,
    RemoveDependency,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptState {
    pub kind: PromptKind,
    pub value: String,
}

impl PromptState {
    #[must_use]
    pub fn new(kind: PromptKind) -> Self {
        Self { kind, value: String::new() }
    }
}
