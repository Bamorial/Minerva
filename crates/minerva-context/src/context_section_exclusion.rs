use crate::{ContextExclusionReason, ContextSectionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSectionExclusion {
    id: ContextSectionId,
    estimated_tokens: usize,
    reason: ContextExclusionReason,
}

impl ContextSectionExclusion {
    #[must_use]
    pub const fn new(
        id: ContextSectionId,
        estimated_tokens: usize,
        reason: ContextExclusionReason,
    ) -> Self {
        Self { id, estimated_tokens, reason }
    }

    #[must_use]
    pub const fn id(&self) -> ContextSectionId {
        self.id
    }

    #[must_use]
    pub const fn estimated_tokens(&self) -> usize {
        self.estimated_tokens
    }

    #[must_use]
    pub const fn reason(&self) -> ContextExclusionReason {
        self.reason
    }
}
