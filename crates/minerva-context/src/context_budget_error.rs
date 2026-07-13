use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextBudgetError {
    CriticalSectionsExceedBudget { budget: usize, required_tokens: usize },
}

impl Display for ContextBudgetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CriticalSectionsExceedBudget { budget, required_tokens } => write!(
                f,
                "critical context requires {required_tokens} estimated tokens, which exceeds the configured budget of {budget}"
            ),
        }
    }
}

impl Error for ContextBudgetError {}
