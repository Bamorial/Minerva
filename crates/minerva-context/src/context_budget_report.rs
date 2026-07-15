use crate::{ContextDocument, ContextManifest, ContextSectionExclusion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextBudgetReport {
    document: ContextDocument,
    budget: usize,
    excluded_sections: Vec<ContextSectionExclusion>,
}

impl ContextBudgetReport {
    #[must_use]
    pub const fn new(
        document: ContextDocument,
        budget: usize,
        excluded_sections: Vec<ContextSectionExclusion>,
    ) -> Self {
        Self { document, budget, excluded_sections }
    }

    #[must_use]
    pub fn excluded_sections(&self) -> &[ContextSectionExclusion] {
        &self.excluded_sections
    }

    #[must_use]
    pub const fn document(&self) -> &ContextDocument {
        &self.document
    }

    #[must_use]
    pub fn manifest(&self) -> ContextManifest {
        ContextManifest::build(
            self.document.sections(),
            ContextDocument::DEFAULT_POLICY,
            self.document.estimation_method(),
            self.document.total_estimated_tokens(),
            Some(self.budget),
            &self.excluded_sections,
        )
    }

    #[must_use]
    pub fn render_with_manifest(&self) -> String {
        self.document.render()
    }
}
