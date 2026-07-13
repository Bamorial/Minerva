use crate::{ProjectValidationFinding, ValidationSeverity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectValidationSummary {
    pub errors: usize,
    pub warnings: usize,
    pub information: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectValidationResult {
    pub findings: Vec<ProjectValidationFinding>,
}

impl ProjectValidationResult {
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.findings.iter().any(|item| item.severity == ValidationSeverity::Error)
    }

    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.findings.iter().any(|item| item.severity == ValidationSeverity::Warning)
    }

    #[must_use]
    pub fn for_task(&self, task_ref: &str) -> Self {
        let findings = self
            .findings
            .iter()
            .filter(|item| item.task_ref.as_deref() == Some(task_ref))
            .cloned()
            .collect();
        Self { findings }
    }

    #[must_use]
    pub fn summary(&self) -> ProjectValidationSummary {
        self.findings.iter().fold(
            ProjectValidationSummary { errors: 0, warnings: 0, information: 0 },
            |mut summary, item| {
                match item.severity {
                    ValidationSeverity::Error => summary.errors += 1,
                    ValidationSeverity::Warning => summary.warnings += 1,
                    ValidationSeverity::Information => summary.information += 1,
                }
                summary
            },
        )
    }
}
