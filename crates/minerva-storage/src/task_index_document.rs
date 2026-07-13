use minerva_domain::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskIndexDocument {
    pub schema_version: u32,
    pub tasks: Vec<Task>,
}

impl TaskIndexDocument {
    pub const SCHEMA_VERSION: u32 = 1;

    #[must_use]
    pub fn from_tasks(tasks: &[Task]) -> Self {
        Self { schema_version: Self::SCHEMA_VERSION, tasks: tasks.to_vec() }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version == Self::SCHEMA_VERSION {
            Ok(())
        } else {
            Err(format!("schema_version must be {}", Self::SCHEMA_VERSION))
        }
    }
}
