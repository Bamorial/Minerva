use crate::{MinervaLayout, task_repository_support};
use minerva_domain::{DeclarationFreshnessProbe, MinervaError, TaskId};

pub fn read_declaration_freshness(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<DeclarationFreshnessProbe, MinervaError> {
    let task = task_repository_support::read_existing(layout, task_id)?;
    Ok(DeclarationFreshnessProbe {
        covered_commit_hash: task.declaration.commit_hash,
        current_commit_hash: None,
    })
}
