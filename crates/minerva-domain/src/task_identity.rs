use crate::TaskId;

#[derive(Debug, Clone)]
pub struct TaskIdentity {
    pub id: TaskId,
    pub title: String,
    pub folder_name: String,
}

impl TaskIdentity {
    #[must_use]
    pub fn new(
        id: TaskId,
        title: impl Into<String>,
        folder_name: impl Into<String>,
    ) -> Self {
        Self { id, title: title.into(), folder_name: folder_name.into() }
    }
}

impl PartialEq for TaskIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TaskIdentity {}
