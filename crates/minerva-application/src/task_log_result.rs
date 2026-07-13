use minerva_domain::{Task, TaskEventKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskLogResult {
    pub task: Task,
    pub filters: Vec<TaskEventKind>,
    pub events: Vec<TaskLogEvent>,
    pub issues: Vec<TaskLogIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskLogReadResult {
    pub events: Vec<TaskLogEvent>,
    pub issues: Vec<TaskLogIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskLogEvent {
    pub id: String,
    pub recorded_at: String,
    pub actor: String,
    pub kind: TaskEventKind,
    pub details: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskLogIssue {
    pub line: usize,
    pub reason: String,
}
