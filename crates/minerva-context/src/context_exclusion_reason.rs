#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextExclusionReason {
    ExcludedToFitBudget,
}

impl ContextExclusionReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExcludedToFitBudget => "excluded_to_fit_budget",
        }
    }
}
