mod support;

use minerva_context::{ContextCompilationRequest, ContextCompilationService};
use minerva_domain::{
    ContextDetail, ContextPolicy, ContextRelationPolicy, RelationshipType,
};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use support::{
    persist_task, refresh_declaration, relate, repo, write_project_instructions,
};

#[test]
fn service_excludes_optional_related_sections_to_fit_budget() {
    let root = repo("budget");
    write_project_instructions(&root, "# Project\n\nKeep context deterministic.");
    let target = persist_task(
        &root,
        1,
        "Target",
        None,
        "# Target\n\nImplement context compilation.",
        "# Declaration\n\nTarget state.",
        &["context compiles"],
    );
    let related = persist_task(
        &root,
        2,
        "Related",
        None,
        "# Related\n\nVery long guidance.\n\nOne.\nTwo.\nThree.\nFour.\nFive.\nSix.",
        "# Declaration\n\nRelated state.",
        &["related handled"],
    );
    relate(&root, target.id, related.id, RelationshipType::References);
    refresh_declaration(&root, target.id, "Target state.");
    let request = ContextCompilationRequest {
        task_ref: target.id.to_string(),
        policy: Some(policy()),
        budget: None,
    };
    let full = ContextCompilationService::compile(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        &root,
        &request,
    )
    .unwrap();
    let related_tokens = full
        .manifest
        .included
        .iter()
        .find(|entry| entry.source == "related_task_summaries")
        .unwrap()
        .estimated_tokens;
    let budgeted = ContextCompilationService::compile(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        &root,
        &ContextCompilationRequest {
            budget: Some(full.estimated_tokens - related_tokens),
            ..request
        },
    )
    .unwrap();
    assert_eq!(budgeted.manifest.budget, Some(full.estimated_tokens - related_tokens));
    assert_eq!(budgeted.manifest.excluded[0].source, "related_task_summaries");
    assert!(!budgeted.markdown.contains("## Related Task Summaries"));
}

fn policy() -> ContextPolicy {
    ContextPolicy {
        project_instructions: Some(ContextDetail::Full),
        target_task_instructions: Some(ContextDetail::Full),
        target_declaration: Some(ContextDetail::Full),
        ancestors: None,
        dependencies: None,
        related_tasks: Some(ContextRelationPolicy {
            detail: ContextDetail::Summary,
            depth: 1,
        }),
        children: None,
        siblings: None,
        include_archived: false,
        include_completed: false,
    }
}
