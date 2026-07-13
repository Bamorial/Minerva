use crate::ContextSection;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContextDocument {
    sections: Vec<ContextSection>,
}

impl ContextDocument {
    #[must_use]
    pub fn new(mut sections: Vec<ContextSection>) -> Self {
        sections.sort_unstable_by_key(|section| section.id);
        Self { sections }
    }

    #[must_use]
    pub fn render(&self) -> String {
        self.sections
            .iter()
            .map(|section| format!("## {}\n\n{}", section.id.heading(), section.body))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
