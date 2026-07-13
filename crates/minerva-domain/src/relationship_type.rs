use crate::TaskId;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    Parent,
    DependsOn,
    Blocks,
    RelatedTo,
    Duplicates,
    Implements,
    References,
}

impl RelationshipType {
    #[must_use]
    pub fn semantic_key(
        self,
        source: TaskId,
        target: TaskId,
    ) -> (TaskId, TaskId, RelationshipType) {
        match self {
            Self::Blocks => (target, source, Self::DependsOn),
            Self::RelatedTo | Self::Duplicates if target < source => {
                (target, source, self)
            }
            _ => (source, target, self),
        }
    }

    #[must_use]
    pub fn dependency_edge(
        self,
        source: TaskId,
        target: TaskId,
    ) -> Option<(TaskId, TaskId)> {
        match self {
            Self::DependsOn => Some((source, target)),
            Self::Blocks => Some((target, source)),
            _ => None,
        }
    }
}
