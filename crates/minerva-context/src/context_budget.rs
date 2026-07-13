use std::{cmp::Reverse, collections::BTreeSet};

use crate::{
    ContextBudgetError, ContextBudgetReport, ContextDocument, ContextExclusionReason,
    ContextSectionExclusion,
};

impl ContextDocument {
    pub fn enforce_budget(
        &self,
        budget: usize,
    ) -> Result<ContextBudgetReport, ContextBudgetError> {
        let required_tokens = self
            .sections()
            .iter()
            .filter(|section| section.id().is_critical())
            .map(crate::ContextSection::estimated_tokens)
            .sum();
        if required_tokens > budget {
            return Err(ContextBudgetError::CriticalSectionsExceedBudget {
                budget,
                required_tokens,
            });
        }
        let mut excluded = Vec::new();
        let mut total = self.total_estimated_tokens();
        let mut optional = self
            .sections()
            .iter()
            .filter(|section| !section.id().is_critical())
            .cloned()
            .collect::<Vec<_>>();
        optional.sort_unstable_by_key(|section| {
            (section.id().budget_priority(), Reverse(section.id()))
        });
        for section in optional {
            if total <= budget {
                break;
            }
            total -= section.estimated_tokens();
            excluded.push(ContextSectionExclusion::new(
                section.id(),
                section.estimated_tokens(),
                ContextExclusionReason::ExcludedToFitBudget,
                section.input_hash().to_owned(),
            ));
        }
        let excluded_ids = excluded
            .iter()
            .map(crate::ContextSectionExclusion::id)
            .collect::<BTreeSet<_>>();
        let included = self
            .sections()
            .iter()
            .filter(|section| !excluded_ids.contains(&section.id()))
            .cloned()
            .collect();
        Ok(ContextBudgetReport::new(ContextDocument::new(included), budget, excluded))
    }
}
