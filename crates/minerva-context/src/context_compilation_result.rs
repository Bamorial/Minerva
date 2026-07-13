use crate::{ContextGraphSelection, ContextManifest, ContextSectionExclusion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextCompilationResult {
    pub markdown: String,
    pub manifest: ContextManifest,
    pub estimated_tokens: usize,
    pub selection: ContextGraphSelection,
    pub excluded_sections: Vec<ContextSectionExclusion>,
}
