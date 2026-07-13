use crate::ContextSelectionItem;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContextGraphSelection {
    pub items: Vec<ContextSelectionItem>,
}

impl ContextGraphSelection {
    #[must_use]
    pub fn task_ids(&self) -> Vec<minerva_domain::TaskId> {
        self.items.iter().map(|item| item.task.id).collect()
    }
}
