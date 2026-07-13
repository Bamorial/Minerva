use crate::{ContextCompilationError, ContextGraphSelection, ContextInclusionReason};
use minerva_application::TaskRepository;
use minerva_domain::{ContextPolicy, DeclarationFreshness, TaskId};
use std::{collections::BTreeSet, path::Path};

pub fn ensure_fresh_declarations(
    task_repo: &impl TaskRepository,
    root: &Path,
    selection: &ContextGraphSelection,
    policy: &ContextPolicy,
) -> Result<(), ContextCompilationError> {
    let mut required = BTreeSet::new();
    if policy.target_declaration.is_some() {
        required.insert(selection.items[0].task.id);
    }
    for item in &selection.items {
        if matches!(
            item.reason,
            ContextInclusionReason::Ancestor { .. }
                | ContextInclusionReason::Dependency { .. }
        ) {
            required.insert(item.task.id);
        }
    }
    for task_id in required {
        ensure_task_is_fresh(task_repo, root, task_id)?;
    }
    Ok(())
}

fn ensure_task_is_fresh(
    task_repo: &impl TaskRepository,
    root: &Path,
    task_id: TaskId,
) -> Result<(), ContextCompilationError> {
    let probe = task_repo.read_declaration_freshness(root, task_id)?;
    let report = probe.evaluate();
    if report.status == DeclarationFreshness::Stale {
        return Err(ContextCompilationError::StaleReference {
            task_ref: task_id.to_string(),
            reasons: report.reasons,
        });
    }
    Ok(())
}
