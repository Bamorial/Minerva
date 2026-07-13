pub const MIXED_TOKEN_ESTIMATION_METHOD: &str = "mixed token estimation methods";

pub trait TokenEstimator {
    fn method(&self) -> &'static str;
    fn estimate(&self, text: &str) -> usize;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ApproximateTokenEstimator;

impl ApproximateTokenEstimator {
    pub const METHOD: &str = "non-whitespace chunks use one token per four characters rounded up; punctuation-only chunks count per character";
}

impl TokenEstimator for ApproximateTokenEstimator {
    fn method(&self) -> &'static str {
        Self::METHOD
    }

    fn estimate(&self, text: &str) -> usize {
        text.split_whitespace().map(estimate_chunk).sum()
    }
}

fn estimate_chunk(chunk: &str) -> usize {
    if chunk.chars().all(|char| char.is_ascii_punctuation()) {
        chunk.chars().count()
    } else {
        chunk.chars().count().div_ceil(4).max(1)
    }
}
