use crate::{ApproximateTokenEstimator, ContextSectionId, TokenEstimator};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSection {
    id: ContextSectionId,
    body: String,
    estimated_tokens: usize,
    estimation_method: &'static str,
    input_hash: String,
}

impl ContextSection {
    #[must_use]
    pub fn new(id: ContextSectionId, body: impl Into<String>) -> Option<Self> {
        Self::new_with_estimator(id, body, &ApproximateTokenEstimator)
    }

    #[must_use]
    pub fn new_with_estimator(
        id: ContextSectionId,
        body: impl Into<String>,
        estimator: &impl TokenEstimator,
    ) -> Option<Self> {
        let body = body.into().trim().to_owned();
        (!body.is_empty()).then(|| Self {
            id,
            estimated_tokens: estimator.estimate(&rendered(id, &body)),
            estimation_method: estimator.method(),
            input_hash: hash_input(&body),
            body,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ContextSectionId {
        self.id
    }

    #[must_use]
    pub fn estimated_tokens(&self) -> usize {
        self.estimated_tokens
    }

    #[must_use]
    pub const fn estimation_method(&self) -> &'static str {
        self.estimation_method
    }

    #[must_use]
    pub fn input_hash(&self) -> &str {
        &self.input_hash
    }

    #[must_use]
    pub fn render(&self) -> String {
        rendered(self.id, &self.body)
    }
}

fn rendered(id: ContextSectionId, body: &str) -> String {
    format!("## {}\n\n{body}", id.heading())
}

fn hash_input(body: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    format!("{:x}", hasher.finalize())
}
