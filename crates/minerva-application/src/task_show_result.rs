use minerva_domain::Task;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaskShowOptions {
    pub include_instructions: bool,
    pub include_declaration: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskShowResult {
    pub task: Task,
    pub parent: Option<TaskShowLink>,
    pub dependencies: Vec<TaskShowLink>,
    pub relationships: Vec<TaskShowRelationship>,
    pub freshness: TaskShowFreshness,
    pub timestamps: TaskShowTimestamps,
    pub instructions: Option<String>,
    pub declaration: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskShowLink {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskShowRelationship {
    pub kind: String,
    pub direction: String,
    pub task: TaskShowLink,
    pub reason: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskShowFreshness {
    pub status: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskShowTimestamps {
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
    pub declaration_updated_at: String,
}
