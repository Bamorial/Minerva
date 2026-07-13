use minerva_context::{
    ContextBudgetError, ContextDocument, ContextSection, ContextSectionId,
    TokenEstimator,
};

#[test]
fn under_budget_documents_are_retained_without_exclusions() {
    let report = document(&[(ContextSectionId::ProjectInstructions, 4)])
        .enforce_budget(8)
        .unwrap();
    assert!(report.excluded_sections().is_empty());
    assert_eq!(report.document().total_estimated_tokens(), 4);
}

#[test]
fn optional_sections_are_excluded_from_lowest_to_highest_priority() {
    let report = document(&[
        (ContextSectionId::ProjectInstructions, 4),
        (ContextSectionId::TargetDeclaration, 4),
        (ContextSectionId::RelatedTaskSummaries, 3),
        (ContextSectionId::AncestorDeclarations, 2),
        (ContextSectionId::DependencyDeclarations, 2),
    ])
    .enforce_budget(10)
    .unwrap();
    let excluded = report.excluded_sections();
    assert_eq!(excluded.len(), 2);
    assert_eq!(excluded[0].id(), ContextSectionId::RelatedTaskSummaries);
    assert_eq!(excluded[1].id(), ContextSectionId::AncestorDeclarations);
    let manifest = report.manifest();
    assert_eq!(manifest.budget, Some(10));
    assert_eq!(manifest.excluded[0].reason, "excluded_to_fit_budget");
    assert_eq!(manifest.input_hashes.len(), 5);
}

#[test]
fn critical_overflow_fails_clearly() {
    let error = document(&[
        (ContextSectionId::ProjectInstructions, 5),
        (ContextSectionId::TargetDeclaration, 4),
    ])
    .enforce_budget(8)
    .unwrap_err();
    assert_eq!(
        error,
        ContextBudgetError::CriticalSectionsExceedBudget {
            budget: 8,
            required_tokens: 9,
        }
    );
    assert_eq!(
        error.to_string(),
        "critical context requires 9 estimated tokens, which exceeds the configured budget of 8"
    );
}

fn document(sections: &[(ContextSectionId, usize)]) -> ContextDocument {
    ContextDocument::new(
        sections
            .iter()
            .map(|(id, tokens)| {
                ContextSection::new_with_estimator(
                    *id,
                    tokens.to_string(),
                    &BodyEstimator,
                )
                .unwrap()
            })
            .collect(),
    )
}

struct BodyEstimator;

impl TokenEstimator for BodyEstimator {
    fn method(&self) -> &'static str {
        "body estimator"
    }

    fn estimate(&self, body: &str) -> usize {
        body.lines().last().unwrap().parse().unwrap()
    }
}
