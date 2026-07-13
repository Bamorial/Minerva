use crate::{ContextExclusionReason, ContextSectionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSectionExclusion {
    id: ContextSectionId,
    estimated_tokens: usize,
    reason: ContextExclusionReason,
    input_hash: String,
}

impl ContextSectionExclusion {
    #[must_use]
    pub fn new(
        id: ContextSectionId,
        estimated_tokens: usize,
        reason: ContextExclusionReason,
        input_hash: String,
    ) -> Self {
        Self { id, estimated_tokens, reason, input_hash }
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

    #[must_use]
    pub fn input_hash(&self) -> &str {
        &self.input_hash
    }
}
