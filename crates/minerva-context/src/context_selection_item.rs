use crate::ContextInclusionReason;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSelectionItem {
    pub task: minerva_domain::Task,
    pub reason: ContextInclusionReason,
}
