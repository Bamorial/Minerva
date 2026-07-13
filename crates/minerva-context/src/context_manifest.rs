use crate::{ContextSection, ContextSectionExclusion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextManifest {
    pub version: u8,
    pub policy: String,
    pub estimation_method: String,
    pub total_estimated_tokens: usize,
    pub budget: Option<usize>,
    pub included: Vec<ContextManifestEntry>,
    pub excluded: Vec<ContextManifestEntry>,
    pub input_hashes: Vec<ContextInputHash>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextManifestEntry {
    pub source: String,
    pub reason: String,
    pub estimated_tokens: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextInputHash {
    pub source: String,
    pub sha256: String,
}

impl ContextManifest {
    #[must_use]
    pub fn build(
        sections: &[ContextSection],
        policy: &str,
        estimation_method: &str,
        total_estimated_tokens: usize,
        budget: Option<usize>,
        excluded: &[ContextSectionExclusion],
    ) -> Self {
        Self {
            version: 1,
            policy: policy.to_owned(),
            estimation_method: estimation_method.to_owned(),
            total_estimated_tokens,
            budget,
            included: sections.iter().map(ContextManifestEntry::included).collect(),
            excluded: excluded.iter().map(ContextManifestEntry::excluded).collect(),
            input_hashes: input_hashes(sections, excluded),
        }
    }

    #[must_use]
    pub fn render_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap().trim().to_owned()
    }
}

impl ContextManifestEntry {
    fn included(section: &ContextSection) -> Self {
        Self::new(
            section.id().source_key(),
            section.id().inclusion_reason(),
            section.estimated_tokens(),
        )
    }

    fn excluded(section: &ContextSectionExclusion) -> Self {
        Self::new(
            section.id().source_key(),
            section.reason().as_str(),
            section.estimated_tokens(),
        )
    }

    fn new(source: &str, reason: &str, estimated_tokens: usize) -> Self {
        Self { source: source.to_owned(), reason: reason.to_owned(), estimated_tokens }
    }
}

fn input_hashes(
    included: &[ContextSection],
    excluded: &[ContextSectionExclusion],
) -> Vec<ContextInputHash> {
    included
        .iter()
        .map(|section| (section.id().source_key(), section.input_hash()))
        .chain(
            excluded
                .iter()
                .map(|section| (section.id().source_key(), section.input_hash())),
        )
        .map(|(source, sha256)| ContextInputHash {
            source: source.to_owned(),
            sha256: sha256.to_owned(),
        })
        .collect()
}
