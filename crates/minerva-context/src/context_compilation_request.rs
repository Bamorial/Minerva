use minerva_domain::ContextPolicy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextCompilationRequest {
    pub task_ref: String,
    pub policy: Option<ContextPolicy>,
    pub budget: Option<usize>,
}

impl ContextCompilationRequest {
    #[must_use]
    pub fn new(task_ref: impl Into<String>) -> Self {
        Self { task_ref: task_ref.into(), policy: None, budget: None }
    }
}
