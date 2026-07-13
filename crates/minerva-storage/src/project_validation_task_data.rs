use minerva_domain::{Relationship, Task};

pub struct TaskValidationData {
    pub tasks: Vec<Task>,
    pub relationships: Vec<Relationship>,
}

impl TaskValidationData {
    #[must_use]
    pub const fn new() -> Self {
        Self { tasks: Vec::new(), relationships: Vec::new() }
    }
}
