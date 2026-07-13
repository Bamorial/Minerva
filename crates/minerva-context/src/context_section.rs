use crate::ContextSectionId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSection {
    pub id: ContextSectionId,
    pub body: String,
}

impl ContextSection {
    #[must_use]
    pub fn new(id: ContextSectionId, body: impl Into<String>) -> Option<Self> {
        let body = body.into().trim().to_owned();
        (!body.is_empty()).then_some(Self { id, body })
    }
}
