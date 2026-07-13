use crate::{
    ApproximateTokenEstimator, ContextSection, ContextSectionId,
    MIXED_TOKEN_ESTIMATION_METHOD, render_context_manifest,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContextDocument {
    sections: Vec<ContextSection>,
    estimation_method: &'static str,
    total_estimated_tokens: usize,
}

impl ContextDocument {
    #[must_use]
    pub fn new(mut sections: Vec<ContextSection>) -> Self {
        sections.sort_unstable_by_key(ContextSection::id);
        Self {
            total_estimated_tokens: sections
                .iter()
                .map(ContextSection::estimated_tokens)
                .sum(),
            estimation_method: estimation_method(&sections),
            sections,
        }
    }

    #[must_use]
    pub fn render(&self) -> String {
        self.sections
            .iter()
            .map(ContextSection::render)
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    #[must_use]
    pub fn render_with_manifest(&self) -> String {
        let body = self.render();
        let manifest = render_context_manifest(
            &self.sections,
            self.estimation_method,
            self.total_estimated_tokens,
            None,
            &[],
        );
        let heading = ContextSectionId::ContextManifestSummary.heading();
        if body.is_empty() {
            format!("## {heading}\n\n{manifest}")
        } else {
            format!("{body}\n\n## {heading}\n\n{manifest}")
        }
    }

    #[must_use]
    pub const fn total_estimated_tokens(&self) -> usize {
        self.total_estimated_tokens
    }

    #[must_use]
    pub const fn estimation_method(&self) -> &'static str {
        self.estimation_method
    }

    #[must_use]
    pub fn sections(&self) -> &[ContextSection] {
        &self.sections
    }
}

fn estimation_method(sections: &[ContextSection]) -> &'static str {
    match sections.first().map(ContextSection::estimation_method) {
        Some(method)
            if sections.iter().all(|section| section.estimation_method() == method) =>
        {
            method
        }
        Some(_) => MIXED_TOKEN_ESTIMATION_METHOD,
        None => ApproximateTokenEstimator::METHOD,
    }
}
