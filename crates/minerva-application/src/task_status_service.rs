use crate::{
    ProjectRepository, TaskRepository, git_support::git_head, render_task_facts,
};
use minerva_domain::{
    DeclarationFreshness, DeclarationFreshnessReason, DeclarationFreshnessReport,
    MinervaError, Task,
};
use std::path::Path;

pub struct TaskStatusService;

impl TaskStatusService {
    pub fn show(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        task_ref: &str,
    ) -> Result<String, MinervaError> {
        let root = project_repo.locate_project_root(start)?;
        let task = task_repo.resolve_task(&root, task_ref)?;
        let mut probe = task_repo.read_declaration_freshness(&root, task.id)?;
        probe.current_commit_hash = git_head(&root);
        Ok(render(&task, &probe.evaluate()))
    }
}

fn render(task: &Task, freshness: &DeclarationFreshnessReport) -> String {
    let reasons = freshness.reasons.iter().map(reason).collect::<Vec<_>>();
    let reasons = if reasons.is_empty() { "none".into() } else { reasons.join(", ") };
    [
        format!("{} {}", task.id, task.title),
        format!("status: {}", task.status),
        format!("version: {}", task.version.get()),
        format!("declaration version: {}", task.declaration.version),
        format!("declaration freshness: {}", status(freshness.status)),
        format!("freshness reasons: {reasons}"),
        render_task_facts(task),
    ]
    .join("\n")
}

fn status(value: DeclarationFreshness) -> &'static str {
    match value {
        DeclarationFreshness::Fresh => "fresh",
        DeclarationFreshness::PotentiallyStale => "potentially-stale",
        DeclarationFreshness::Stale => "stale",
        DeclarationFreshness::Unknown => "unknown",
    }
}

fn reason(value: &DeclarationFreshnessReason) -> &'static str {
    match value {
        DeclarationFreshnessReason::MissingCoveredCommit => "missing-covered-commit",
        DeclarationFreshnessReason::CoveredCommitUnavailable => {
            "covered-commit-unavailable"
        }
        DeclarationFreshnessReason::CoveredCommitDiffers => "covered-commit-differs",
        DeclarationFreshnessReason::InstructionsUpdatedAfterDeclaration => {
            "instructions-updated-after-declaration"
        }
        DeclarationFreshnessReason::RelationshipsUpdatedAfterDeclaration => {
            "relationships-updated-after-declaration"
        }
        DeclarationFreshnessReason::TaskMetadataUpdatedAfterDeclaration => {
            "task-metadata-updated-after-declaration"
        }
    }
}
